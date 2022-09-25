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
        .insert_resource(WindowDescriptor {
            width: 960.,
            height: 540.,
            title: "Rusted Chrome".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
