// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use rusted_chrome::GamePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 1280.,
                        height: 800.,
                        title: "Rusted Chrome".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        // mode: bevy::window::WindowMode::BorderlessFullscreen,
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(GamePlugin)
        .run();
}
