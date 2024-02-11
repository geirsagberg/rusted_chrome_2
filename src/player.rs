use std::time::Duration;

use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::hashbrown::HashMap;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::animation::{Animation, Clip};
use crate::camera::CameraTarget;
use crate::components::aiming::{Aiming, AimingChild};
use crate::components::facing::Facing;
use crate::loading::TextureAssets;
use crate::{GameState, PlayerAction, PHYSICS_STEP};

pub struct PlayerPlugin;

#[derive(Component, Default)]
pub struct Player {}

#[derive(Component)]
pub struct Standing {
    pub is_standing: bool,
}

#[derive(Component, Reflect, Default, Clone, Debug)]
pub struct Health {
    pub health: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Self { health }
    }
}

#[derive(Component, Reflect, Default, Clone, Debug)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component, Reflect, Default)]
pub struct Gun {
    pub shot_timer: Timer,
}

impl Lifetime {
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
        }
    }
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
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(
            Update,
            (
                move_player,
                animate_player,
                change_aim,
                rotate_aim_children,
                check_if_standing,
                shoot,
                gun_time,
                lifetime_cleanup,
                health_remove_on_hit,
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn health_remove_on_hit(query: Query<&Health>) {
    for health in &query {
        if health.health <= 0. {
            println!("Player died");
        }
    }
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

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut input_map = InputMap::default();

    input_map
        .insert(KeyCode::J, PlayerAction::Shoot)
        .insert(MouseButton::Left, PlayerAction::Shoot)
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .insert(KeyCode::Space, PlayerAction::Jump);
    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                anchor: Anchor::Custom(vec2(0., -0.125)),
                ..default()
            },
            texture_atlas: textures.cyborg.clone(),
            transform: Transform::from_xyz(480., 256., 3.),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: textures.hand.clone(),
                    transform: Transform::from_xyz(4., 4., -0.1),
                    ..default()
                })
                .insert(AimingChild)
                .with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: textures.gun.clone(),
                            transform: Transform::from_xyz(12., 2., 0.05),
                            ..default()
                        })
                        .insert(Gun {
                            shot_timer: Timer::from_seconds(0.1, TimerMode::Once),
                        });
                });
        })
        .insert(Animation::new(
            0.12,
            HashMap::from([
                (
                    String::from("idle"),
                    Clip {
                        frames: 0..3,
                        repeat: true,
                    },
                ),
                (
                    String::from("jumping"),
                    Clip {
                        frames: 6..9,
                        repeat: false,
                    },
                ),
                (
                    String::from("running"),
                    Clip {
                        frames: 12..17,
                        repeat: true,
                    },
                ),
                (
                    String::from("walking"),
                    Clip {
                        frames: 24..29,
                        repeat: true,
                    },
                ),
            ]),
        ))
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule_y(8., 8.))
        .insert(ColliderScale::Absolute(vec2(1., 1.)))
        .insert(ColliderMassProperties::Mass(80.0))
        .insert(Ccd { enabled: true })
        .insert(Facing::Right)
        .insert(CameraTarget::with_radius(100.))
        .insert(Aiming::default())
        .insert(Standing::default())
        .insert(Player::default())
        .insert(Health::new(100.))
        .insert(Velocity::linear(vec2(0., 0.)))
        .insert(InputManagerBundle::<PlayerAction> {
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
            velocity.linvel.y = 384.;
        };
    }
}

fn gun_time(mut query: Query<&mut Gun>) {
    let delta = Duration::from_secs_f32(PHYSICS_STEP);

    for mut gun in &mut query {
        gun.shot_timer.tick(delta);
    }
}

fn shoot(
    mut commands: Commands,
    query: Query<(&Children, &Velocity, &ActionState<PlayerAction>), With<Player>>,
    arm_query: Query<&Children, With<AimingChild>>,
    mut gun_query: Query<(&GlobalTransform, &mut Gun)>,
    textures: Res<TextureAssets>,
) {
    let bullet_speed = 400.;
    for (children, velocity, action_state) in &query {
        if action_state.pressed(PlayerAction::Shoot) {
            let arm_children = arm_query.get(children[0]).unwrap();
            let (gun_transform, mut gun) = gun_query.get_mut(arm_children[0]).unwrap();

            if gun.shot_timer.finished() {
                gun.shot_timer.reset();

                let transform = gun_transform
                    .compute_transform()
                    .mul_transform(Transform::from_xyz(2., 0., 0.1));

                let forward = transform
                    .rotation
                    .mul_vec3(vec3(transform.scale.x, 0., 0.))
                    .normalize()
                    .truncate();

                commands
                    .spawn(SpriteBundle {
                        texture: textures.bullet.clone(),
                        transform: Transform::from_translation(transform.translation)
                            .with_scale(Vec3::splat(2.)),
                        ..default()
                    })
                    .insert(RigidBody::Dynamic)
                    .insert(Collider::ball(1.))
                    .insert(LockedAxes::ROTATION_LOCKED)
                    .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1))
                    .insert(Ccd { enabled: true })
                    .insert(Restitution {
                        coefficient: 1.,
                        combine_rule: CoefficientCombineRule::Max,
                    })
                    .insert(Friction {
                        coefficient: 0.,
                        combine_rule: CoefficientCombineRule::Min,
                    })
                    .insert(GravityScale(0.))
                    .insert(Lifetime::from_seconds(2.0))
                    .insert(Velocity::linear(
                        forward * bullet_speed + velocity.linvel * 0.5,
                    ));
            }
        }
    }
}

fn lifetime_cleanup(mut commands: Commands, mut query: Query<(Entity, &mut Lifetime)>) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(Duration::from_secs_f32(PHYSICS_STEP));
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

const AIMING_SPEED: f32 = 2.;

fn change_aim(mut query: Query<(&mut Aiming, &ActionState<PlayerAction>)>) {
    for (mut aiming, action_state) in &mut query {
        let axis_pair = action_state
            .axis_pair(PlayerAction::Move)
            .unwrap_or_default();

        if axis_pair.y() > 0.1 || axis_pair.y() < -0.1 {
            aiming.angle += axis_pair.y() * AIMING_SPEED * PHYSICS_STEP;
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
