use bevy::core::{Pod, Zeroable};

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_ggrs::GGRSPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::{
    ExternalForce, ExternalImpulse, GravityScale, NoUserData, PhysicsStages, RapierConfiguration,
    RapierPhysicsPlugin, TimestepMode, Velocity,
};
use bevy_rapier2d::rapier::prelude::IntegrationParameters;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use camera::CameraPlugin;
use components::aiming::Aiming;
use components::facing::Facing;
use debug::DebugPlugin;
use fps::FpsPlugin;
use ggrs::{Config, PlayerHandle};
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;

use animation::AnimationPlugin;
use leafwing_input_manager::systems::{release_on_disable, tick_action_state, update_action_state};
use loading::LoadingPlugin;
use platforms::PlatformsPlugin;
use player::{get_player_rollback_systems, Gun, Lifetime, Player, PlayerPlugin};
use tiled_map::TiledMapPlugin;
use world::{get_world_rollback_systems, WorldPlugin};

mod animation;
mod atlas_data;
mod camera;
mod components;
mod debug;
mod fps;
mod loading;
mod platforms;
mod player;
mod screen_diags;
mod tiled_map;
mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
}

pub struct GamePlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerAction {
    Move,
    Aim,
    Jump,
    Shoot,
}

const PIXELS_PER_METER: f32 = 64.;

const NORMALIZED_FPS: f32 = 60.;

const PHYSICS_FPS: usize = 60;

const PHYSICS_STEP: f32 = (NORMALIZED_FPS / PHYSICS_FPS as f32) / NORMALIZED_FPS;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::GRAY))
            .add_loopless_state(GameState::Loading)
            .add_plugin(TilemapPlugin)
            .add_plugin(TiledMapPlugin)
            .add_plugin(LoadingPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(PlatformsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(FpsPlugin)
            .add_plugin(RollbackPlugin)
            .add_plugin(DebugLinesPlugin::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(DebugPlugin)
            .add_plugin(
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER)
                    .with_default_system_setup(false),
            )
            .insert_resource(RapierConfiguration {
                gravity: vec2(0., -9.81 * PIXELS_PER_METER),
                timestep_mode: TimestepMode::Fixed {
                    dt: 1. / PHYSICS_FPS as f32,
                    substeps: 1,
                },
                ..default()
            })
            .insert_resource(IntegrationParameters {
                max_ccd_substeps: 5,
                ..default()
            })
            .init_resource::<ToggleActions<PlayerAction>>()
            .init_resource::<ClashStrategy>();

        #[cfg(debug_assertions)]
        {
            // app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn flip_facing(mut query: Query<(&mut Transform, &Facing)>) {
    for (mut transform, facing) in &mut query {
        transform.scale.x = if facing.is_left() { -1. } else { 1. };
    }
}

struct RollbackPlugin;

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = InputBits;
    type State = u8;
    type Address = String;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct InputBits {
    pub input: u8,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
enum RollbackStage {
    PreUpdate,
    Update,
    PostUpdate,
}

fn get_input_manager_systems() -> SystemSet {
    SystemSet::new()
        .with_system(
            tick_action_state::<PlayerAction>
                .label(InputManagerSystem::Tick)
                .before(InputManagerSystem::Update),
        )
        .with_system(update_action_state::<PlayerAction>.label(InputManagerSystem::Update))
        .with_system(
            release_on_disable::<PlayerAction>
                .label(InputManagerSystem::ReleaseOnDisable)
                .after(InputManagerSystem::Update),
        )
}

impl Plugin for RollbackPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<GGRSConfig>::new()
            // define frequency of rollback game logic update
            .with_update_frequency(PHYSICS_FPS)
            // define system that returns inputs given a player handle, so GGRS can send the inputs around
            .with_input_system(map_player_input)
            .register_rollback_type::<Transform>()
            .register_rollback_type::<Velocity>()
            .register_rollback_type::<ExternalForce>()
            .register_rollback_type::<ExternalImpulse>()
            .register_rollback_type::<GravityScale>()
            .register_rollback_type::<Aiming>()
            .register_rollback_type::<Lifetime>()
            .register_rollback_type::<Facing>()
            .register_rollback_type::<Gun>()
            // these systems will be executed as part of the advance frame update
            .with_rollback_schedule(
                Schedule::default()
                    .with_stage(
                        RollbackStage::PreUpdate,
                        SystemStage::parallel().with_system_set(get_input_manager_systems()),
                    )
                    .with_stage(
                        RollbackStage::Update,
                        SystemStage::parallel()
                            .with_system_set(get_player_rollback_systems())
                            .with_system_set(get_world_rollback_systems()),
                    )
                    .with_stage(
                        RollbackStage::PostUpdate,
                        SystemStage::parallel()
                            .with_system_set(SystemSet::new().with_system(flip_facing)),
                    )
                    .with_stage(
                        PhysicsStages::SyncBackend,
                        SystemStage::parallel().with_system_set(
                            RapierPhysicsPlugin::<NoUserData>::get_systems(
                                PhysicsStages::SyncBackend,
                            ),
                        ),
                    )
                    .with_stage(
                        PhysicsStages::StepSimulation,
                        SystemStage::parallel().with_system_set(
                            RapierPhysicsPlugin::<NoUserData>::get_systems(
                                PhysicsStages::StepSimulation,
                            ),
                        ),
                    )
                    .with_stage(
                        PhysicsStages::Writeback,
                        SystemStage::parallel().with_system_set(
                            RapierPhysicsPlugin::<NoUserData>::get_systems(
                                PhysicsStages::Writeback,
                            ),
                        ),
                    ),
            )
            .build(app);
    }
}

const INPUT_RIGHT: u8 = 1 << 0;
const INPUT_LEFT: u8 = 1 << 1;
const INPUT_UP: u8 = 1 << 2;
const INPUT_DOWN: u8 = 1 << 3;

fn map_player_input(
    player_handle: In<PlayerHandle>,
    query: Query<(&Player, &ActionState<PlayerAction>)>,
) -> InputBits {
    for (player, action_state) in query.iter() {
        if player.handle == player_handle.0 {
            let mut input = 0;
            if let Some(move_axis) = action_state.clamped_axis_pair(PlayerAction::Move) {
                if move_axis.x() > 0. {
                    input |= INPUT_RIGHT;
                }
                if move_axis.x() < 0. {
                    input |= INPUT_LEFT;
                }
                if move_axis.y() > 0. {
                    input |= INPUT_UP;
                }
                if move_axis.y() < 0. {
                    input |= INPUT_DOWN;
                }
            }
            return InputBits { input };
        }
    }
    panic!("No player found for handle {:?}", player_handle.0);
}
