// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;

use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;
use rusted_chrome::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1280.,
                height: 800.,
                title: "Rusted Chrome".to_string(),
                present_mode: PresentMode::AutoVsync,
                // mode: bevy::window::WindowMode::BorderlessFullscreen,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
