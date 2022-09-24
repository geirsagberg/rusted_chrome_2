use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use crate::{
    screen_diags::{ScreenDiagsPlugin, ScreenDiagsState},
    PHYSICS_FPS,
};

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ScreenDiagsPlugin)
            .add_plugin(FramepacePlugin)
            .insert_resource(FramepaceSettings {
                limiter: Limiter::from_framerate(PHYSICS_FPS as f64),
                ..Default::default()
            })
            .insert_resource(FpsSettings::default())
            .add_system(toggle_framepace_plugin)
            .add_system(toggle_fps);
    }
}

struct FpsSettings {
    fps: u32,
}

impl Default for FpsSettings {
    fn default() -> Self {
        Self { fps: 60 }
    }
}

fn toggle_fps(mut screen_diags_state: ResMut<ScreenDiagsState>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::LControl) && input.just_pressed(KeyCode::F) {
        if screen_diags_state.enabled() {
            screen_diags_state.disable();
        } else {
            screen_diags_state.enable();
        }
    }
}

fn toggle_framepace_plugin(
    mut settings: ResMut<FramepaceSettings>,
    input: Res<Input<KeyCode>>,
    mut app_settings: ResMut<FpsSettings>,
) {
    if input.just_pressed(KeyCode::I) {
        settings.limiter = match settings.limiter {
            Limiter::Auto => Limiter::Off,
            Limiter::Manual(_) => Limiter::Auto,
            Limiter::Off => Limiter::from_framerate(app_settings.fps.into()),
        }
    }
    if input.just_pressed(KeyCode::NumpadAdd) {
        app_settings.fps += 1;
        settings.limiter = Limiter::from_framerate(app_settings.fps.into());
    }
    if input.just_pressed(KeyCode::NumpadSubtract) {
        app_settings.fps -= 1;
        if app_settings.fps == 0 {
            app_settings.fps = 1;
        }
        settings.limiter = Limiter::from_framerate(app_settings.fps.into());
    }
}
