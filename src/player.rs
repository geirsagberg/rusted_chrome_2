use bevy::prelude::*;
use iyes_loopless::prelude::*;
use serde::Deserialize;

use crate::actions::Actions;
use crate::animation::Animation;
use crate::atlas_data::AnimationSpriteSheetMeta;
use crate::GameState;
use crate::loading::TextureAssets;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, PartialEq, Eq, Hash, Clone, Copy, Deserialize)]
pub enum PlayerState {
    Idle,
    Walking,
    Running,
    Jumping,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Playing,
            SystemSet::new().with_system(spawn_player),
        )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(move_player)
                    .into(),
            );
    }
}

#[derive(Bundle)]
pub struct AnimatedSpriteSheetBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    animation: Animation,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    #[bundle]
    animated_sprite_sheet: AnimatedSpriteSheetBundle,
    player: Player,
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>, animated_sprite_sheet_assets: Res<Assets<AnimationSpriteSheetMeta>>) {
    let cyborg = animated_sprite_sheet_assets.get(&textures.cyborg).unwrap();
    let mut animation = Animation::new(
        cyborg.animation_frame_duration,
        cyborg.animations.clone(),
    );
    animation.play("idle", true);
    commands
        .spawn_bundle(
            PlayerBundle {
                animated_sprite_sheet: AnimatedSpriteSheetBundle {
                    sprite_sheet: SpriteSheetBundle {
                        sprite: default(),
                        texture_atlas: cyborg.atlas_handle.clone(),
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..default()
                    },
                    animation,
                },
                player: Player,
            }
        );
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in player_query.iter_mut() {
        player_transform.translation += movement;
    }
}
