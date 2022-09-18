use bevy::core::{Pod, Zeroable};

#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::{app::App, render::texture::ImageSettings};
use bevy_ggrs::GGRSPlugin;
use bevy_rapier2d::prelude::{
    AdditionalMassProperties, ExternalForce, ExternalImpulse, GravityScale, NoUserData,
    PhysicsStages, RapierConfiguration, RapierPhysicsPlugin, TimestepMode, Velocity,
};
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use fps::FpsPlugin;
use ggrs::{Config, PlayerHandle};
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;

use animation::AnimationPlugin;
use leafwing_input_manager::systems::{release_on_disable, tick_action_state, update_action_state};
use loading::LoadingPlugin;
use platforms::PlatformsPlugin;
use player::{get_player_rollback_systems, Player, PlayerPlugin};
use world::{get_world_rollback_systems, WorldPlugin};

mod animation;
mod atlas_data;
mod components;
mod fps;
mod loading;
mod platforms;
mod player;
mod screen_diags;
mod world;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
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
    Jump,
}

const PIXELS_PER_METER: f32 = 32.;

const PHYSICS_FPS: usize = 60;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(ImageSettings::default_nearest())
            .add_loopless_state(GameState::Loading)
            .add_startup_system(setup_camera)
            .add_plugin(LoadingPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PlatformsPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(FpsPlugin)
            .add_plugin(RollbackPlugin)
            .add_plugin(RapierDebugRenderPlugin::default())
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
            .init_resource::<ToggleActions<PlayerAction>>()
            .init_resource::<ClashStrategy>();

        #[cfg(debug_assertions)]
        {
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 1.;
    commands.spawn_bundle(bundle);
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
            .register_rollback_type::<AdditionalMassProperties>()
            .register_rollback_type::<ExternalForce>()
            .register_rollback_type::<ExternalImpulse>()
            .register_rollback_type::<GravityScale>()
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
