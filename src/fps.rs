use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};

use crate::screen_diags::ScreenDiagsPlugin;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ScreenDiagsPlugin)
            .add_plugin(FramepacePlugin)
            .insert_resource(FramepaceSettings {
                limiter: Limiter::Off,
                ..Default::default()
            })
            .insert_resource(FpsSettings::default())
            .add_system(toggle_framepace_plugin);
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
