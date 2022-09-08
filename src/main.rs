// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::DefaultPlugins;

use bevy::window::PresentMode;
use bevy_framepace::FramepacePlugin;
use bevy_framepace::FramepaceSettings;
use rusted_chrome::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Rusted Chrome".to_string(),
            present_mode: PresentMode::AutoNoVsync,
            ..Default::default()
        })
        // .add_plugin(FramepacePlugin)
        // .insert_resource(FramepaceSettings {
        //     limiter: bevy_framepace::Limiter::Auto,
        //     ..Default::default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
