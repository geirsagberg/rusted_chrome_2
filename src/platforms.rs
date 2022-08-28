use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use heron::{CollisionShape, Gravity, PhysicMaterial, RigidBody};

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_platforms)
            .insert_resource(Gravity::from(vec3(0., -4000., 0.)));
    }
}

fn create_platforms(mut commands: Commands) {
    let size = vec2(400., 20.);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform::from_translation(vec3(0., -100., 0.)),
            ..default()
        })
        .insert(PhysicMaterial::default())
        .insert(RigidBody::Static)
        .insert(CollisionShape::Cuboid {
            half_extends: size.extend(0.) / 2.,
            border_radius: None,
        });
}
