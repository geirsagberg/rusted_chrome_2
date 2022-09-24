use bevy::prelude::*;
use bevy_rapier2d::render::DebugRenderContext;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(toggle_rapier_debug);
    }
}

fn toggle_rapier_debug(
    mut rapier_debug_context: ResMut<DebugRenderContext>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::F1) {
        rapier_debug_context.enabled = !rapier_debug_context.enabled;
    }
}
