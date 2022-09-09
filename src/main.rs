// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use bevy::prelude::*;
use bevy::DefaultPlugins;

use bevy::window::PresentMode;
use bevy_framepace::FramepacePlugin;
use bevy_framepace::FramepaceSettings;
use bevy_framepace::Limiter;
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
        // .insert_resource(FramepaceSettings {
        //     limiter: bevy_framepace::Limiter::Auto,
        //     ..Default::default()
        // })
        .add_plugins(DefaultPlugins)
        .add_plugin(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::Off,
            ..Default::default()
        })
        .add_system(toggle_plugin)
        .add_plugin(GamePlugin)
        .run();
}

fn toggle_plugin(mut settings: ResMut<FramepaceSettings>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::I) {
        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Manual(_) => Limiter::Auto,
            Limiter::Off => Limiter::from_framerate(60.),
        }
    }
}
