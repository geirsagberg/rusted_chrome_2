#![allow(unused_parens)]
use bevy::app::App;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use blood::BloodPlugin;
use camera::CameraPlugin;
use components::facing::Facing;
use debug::DebugPlugin;
use fps::FpsPlugin;
use leafwing_input_manager::prelude::*;

use animation::AnimationPlugin;
use loading::LoadingPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod animation;
mod blood;
mod camera;
mod components;
mod debug;
mod fps;
mod loading;
mod platforms;
mod player;
mod screen_diags;
mod world;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Copy, States, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub struct GamePlugin;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
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
            .add_state::<GameState>()
            .add_plugins(LdtkPlugin)
            .add_plugins(LoadingPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(WorldPlugin)
            .add_plugins(AnimationPlugin)
            .add_plugins(PlatformsPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(FpsPlugin)
            .add_plugins(BloodPlugin)
            .add_plugins(leafwing_input_manager::prelude::InputManagerPlugin::<
                PlayerAction,
            >::default())
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(DebugPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                PIXELS_PER_METER,
            ))
            .insert_resource(RapierConfiguration {
                gravity: vec2(0., -9.81 * PIXELS_PER_METER),
                timestep_mode: TimestepMode::Fixed {
                    dt: 1. / PHYSICS_FPS as f32,
                    substeps: 1,
                },
                ..default()
            })
            .add_systems(PostUpdate, flip_facing)
            .init_resource::<ToggleActions<PlayerAction>>()
            .init_resource::<ClashStrategy>();
    }
}

fn flip_facing(mut query: Query<(&mut Transform, &Facing)>) {
    for (mut transform, facing) in &mut query {
        transform.scale.x = if facing.is_left() { -1. } else { 1. };
    }
}
