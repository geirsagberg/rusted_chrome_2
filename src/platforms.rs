use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_platforms);
    }
}

fn create_platforms(mut commands: Commands) {
    let size = vec2(1200., 20.);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(vec3(0., -210., 0.)),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size.x / 2., size.y / 2.));
}
