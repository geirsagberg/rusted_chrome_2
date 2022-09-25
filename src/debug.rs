use bevy::prelude::*;
use bevy_rapier2d::render::DebugRenderContext;

use crate::screen_diags::ScreenDiagsState;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(set_defaults)
            .add_system(toggle_rapier_debug)
            .add_system(toggle_fps);
    }
}

fn set_defaults(
    mut debug_render_context: ResMut<DebugRenderContext>,
    mut screen_diags_state: ResMut<ScreenDiagsState>,
) {
    debug_render_context.enabled = false;
    screen_diags_state.disable();
}

fn toggle_rapier_debug(
    mut rapier_debug_context: ResMut<DebugRenderContext>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F1) {
        rapier_debug_context.enabled = !rapier_debug_context.enabled;
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
