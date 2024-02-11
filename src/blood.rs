use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::dynamics::RigidBodyMassProps};
use rand::{random, Rng};

use crate::player::{Bullet, Lifetime, Player};

pub struct BloodPlugin;

impl Plugin for BloodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_blood_on_hit);
    }
}

fn spawn_blood_on_hit(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    bullet_query: Query<(Entity, &Transform, &Velocity), With<Bullet>>,
    player_query: Query<&Player>,
) {
    for collision_event in events.read() {
        match collision_event {
            CollisionEvent::Started(first, second, _) => {
                let bullet = bullet_query.get(*first).or(bullet_query.get(*second));
                let player = player_query.get(*first).or(player_query.get(*second));
                if let (Ok((entity, transform, velocity)), Ok(_)) = (bullet, player) {
                    let mut rng = rand::thread_rng();
                    for _ in 0..100 {
                        // Generate a random angle within 45 degrees in radians
                        let angle = rng.gen_range(-PI / 8.0..=PI / 8.0);

                        // Extract the x and y components of the velocity
                        let x = velocity.linvel.x;
                        let y = velocity.linvel.y;

                        // Calculate the new x and y components using the rotation matrix formula
                        let new_x = x * angle.cos() - y * angle.sin();
                        let new_y = x * angle.sin() + y * angle.cos();

                        // Create a new Velocity object with the new x and y components
                        let new_velocity = Velocity::linear(
                            Vec2::new(new_x, new_y) * (random::<f32>() * 0.5 + 0.5),
                        );
                        commands.spawn((
                            SpriteBundle {
                                transform: transform.clone(),
                                sprite: Sprite {
                                    color: Color::RED,
                                    custom_size: Some(Vec2::splat(1.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            Blood,
                            Lifetime::from_seconds(3.),
                            RigidBody::Dynamic,
                            AdditionalMassProperties::Mass(0.1),
                            new_velocity,
                        ));
                    }
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct Blood;
