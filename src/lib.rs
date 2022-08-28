#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::{app::App, render::texture::ImageSettings};
use heron::PhysicsPlugin;
use iyes_loopless::prelude::AppLooplessStateExt;
use leafwing_input_manager::prelude::*;

use animation::AnimationPlugin;
use loading::LoadingPlugin;
use platforms::PlatformsPlugin;
use player::PlayerPlugin;

mod animation;
mod atlas_data;
pub mod components;
mod loading;
mod platforms;
mod player;

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
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .insert_resource(ImageSettings::default_nearest())
            .add_loopless_state(GameState::Loading)
            .add_startup_system(setup_camera)
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(LoadingPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PlatformsPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(AnimationPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(LogDiagnosticsPlugin::default());
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        }
    }
}

fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle::default();
    bundle.projection.scale = 0.5;
    commands.spawn_bundle(bundle);
}
