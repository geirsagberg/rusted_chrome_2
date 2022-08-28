use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use heron::{CollisionShape, RigidBody, RotationConstraints, Velocity};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::animation::Animation;
use crate::atlas_data::AnimationSpriteSheetMeta;
use crate::components::facing::Facing;
use crate::loading::TextureAssets;
use crate::{GameState, PlayerAction};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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
                .with_system(animate_player)
                .into(),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animated_sprite_sheet_assets: Res<Assets<AnimationSpriteSheetMeta>>,
) {
    let cyborg = animated_sprite_sheet_assets.get(&textures.cyborg).unwrap();
    let mut animation = Animation::new(cyborg.animation_frame_duration, cyborg.animations.clone());
    animation.play("idle", true);
    let mut input_map = InputMap::default();

    input_map.insert(
        VirtualDPad {
            up: KeyCode::W.into(),
            down: KeyCode::S.into(),
            left: KeyCode::A.into(),
            right: KeyCode::D.into(),
        },
        PlayerAction::Move,
    );
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::Custom(vec2(0., -0.125)),
                ..default()
            },
            texture_atlas: cyborg.atlas_handle.clone(),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(RotationConstraints::lock())
        .insert(CollisionShape::Capsule {
            half_segment: 8.,
            radius: 8.,
        })
        .insert(Facing::Right)
        .insert(animation)
        .insert(Player)
        .insert(Velocity::from_linear(vec3(0., 0., 0.)))
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        });
}

fn move_player(mut player_query: Query<(&mut Velocity, &ActionState<PlayerAction>), With<Player>>) {
    let speed = 150.;

    for (mut velocity, action_state) in player_query.iter_mut() {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();
        velocity.linear = vec3(axis_pair.x(), axis_pair.y(), 0.) * speed;
    }
}

fn animate_player(
    mut player_query: Query<(&mut Animation, &ActionState<PlayerAction>), With<Player>>,
) {
    for (mut animation, action_state) in player_query.iter_mut() {
        if action_state.just_pressed(PlayerAction::Move) {
            animation.play("running", true)
        } else if action_state.just_released(PlayerAction::Move) {
            animation.play("idle", true)
        }
    }
}
