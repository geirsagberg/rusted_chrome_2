use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier2d::prelude::*;
use ggrs::{PlayerHandle, PlayerType, SessionBuilder};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::animation::Animation;
use crate::atlas_data::AnimationSpriteSheetMeta;
use crate::components::aiming::{Aiming, AimingChild};
use crate::components::facing::Facing;
use crate::loading::TextureAssets;
use crate::{GGRSConfig, GameState, PlayerAction, PHYSICS_FPS, PIXELS_PER_METER};

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player {
    pub handle: PlayerHandle,
}

#[derive(Component)]
pub struct Standing {
    pub is_standing: bool,
}

impl Default for Standing {
    fn default() -> Self {
        Self { is_standing: false }
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            GameState::Playing,
            SystemSet::new()
                .with_system(spawn_player)
                .with_system(start_session),
        );
    }
}

pub fn get_player_rollback_systems() -> SystemSet {
    ConditionSet::new()
        .run_in_state(GameState::Playing)
        .with_system(move_player)
        .with_system(animate_player)
        .with_system(change_aim)
        .with_system(rotate_aim_children)
        .with_system(check_if_standing)
        .into()
}

fn start_session(mut commands: Commands) {
    let session = SessionBuilder::<GGRSConfig>::new()
        .with_num_players(1)
        .with_fps(PHYSICS_FPS)
        .expect("Invalid FPS")
        .add_player(PlayerType::Local, 0)
        .expect("Could not add local player")
        .start_synctest_session()
        .expect("");

    commands.insert_resource(session);
    commands.insert_resource(SessionType::SyncTestSession);
}

fn check_if_standing(
    mut query: Query<(&Transform, &Collider, &mut Standing)>,
    rapier_context: Res<RapierContext>,
) {
    for (transform, collider, mut standing) in &mut query {
        let position = transform.translation;
        let filter = QueryFilter::only_fixed();

        let distance_down = collider.raw.compute_local_aabb().half_extents().y + 1.;

        if let Some((_, _)) = rapier_context.cast_ray(
            position.truncate(),
            -transform.local_y().truncate(),
            distance_down,
            true,
            filter,
        ) {
            standing.is_standing = true;
        } else {
            standing.is_standing = false;
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    animated_sprite_sheet_assets: Res<Assets<AnimationSpriteSheetMeta>>,
    mut rollback_id_provider: ResMut<RollbackIdProvider>,
) {
    let cyborg = animated_sprite_sheet_assets.get(&textures.cyborg).unwrap();
    let mut animation = Animation::new(cyborg.animation_frame_duration, cyborg.animations.clone());
    animation.play("idle", true);
    let mut input_map = InputMap::default();

    input_map
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .insert(KeyCode::Space, PlayerAction::Jump);
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
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    texture: textures.hand.clone(),
                    transform: Transform::from_xyz(4., 4., -0.1),
                    ..default()
                })
                .insert(AimingChild);
        })
        .insert(Rollback::new(rollback_id_provider.next_id()))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule_y(8., 8.))
        .insert(ColliderMassProperties::Mass(80.0))
        .insert(Facing::Right)
        .insert(Aiming::default())
        .insert(Standing::default())
        .insert(animation)
        .insert(Player::default())
        .insert(Velocity::linear(vec2(0., 0.)))
        .insert_bundle(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        });
}

fn move_player(
    mut query: Query<
        (
            &mut Velocity,
            &ActionState<PlayerAction>,
            &mut Facing,
            &Standing,
        ),
        With<Player>,
    >,
) {
    let speed = 150.;

    for (mut velocity, action_state, mut facing, standing) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();

        if axis_pair.x() > 0.1 {
            facing.set(Facing::Right);
        } else if axis_pair.x() < -0.1 {
            facing.set(Facing::Left);
        }

        velocity.linvel.x = axis_pair.x() * speed;

        if action_state.just_pressed(PlayerAction::Jump) && standing.is_standing {
            velocity.linvel.y = 6. * PIXELS_PER_METER;
        };
    }
}

fn change_aim(mut query: Query<(&mut Aiming, &ActionState<PlayerAction>)>, time: Res<Time>) {
    for (mut aiming, action_state) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();

        if axis_pair.y() > 0.1 || axis_pair.y() < -0.1 {
            aiming.angle += axis_pair.y() * time.delta_seconds();
            aiming.angle = aiming.angle.clamp(aiming.min_angle, aiming.max_angle);
        }
    }
}

fn rotate_aim_children(
    query: Query<(&Aiming, &Children)>,
    mut aim_query: Query<&mut Transform, With<AimingChild>>,
) {
    for (aiming, children) in &query {
        for &child in children {
            if let Ok(mut transform) = aim_query.get_mut(child) {
                transform.rotation = Quat::from_rotation_z(aiming.angle);
            }
        }
    }
}

fn animate_player(
    mut query: Query<(&mut Animation, &ActionState<PlayerAction>, &Standing), With<Player>>,
) {
    for (mut animation, action_state, standing) in &mut query {
        if standing.is_standing {
            if action_state
                .axis_pair(PlayerAction::Move)
                .unwrap_or_default()
                .x()
                != 0.
                && animation.current_animation != Some(String::from("running"))
            {
                animation.play("running", true)
            } else if action_state.just_released(PlayerAction::Move)
                || animation.current_animation == Some(String::from("jumping"))
            {
                animation.play("idle", true)
            }
        } else if animation.current_animation != Some(String::from("jumping")) {
            animation.play("jumping", false)
        }
    }
}
