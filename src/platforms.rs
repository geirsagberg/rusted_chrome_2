use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use bevy_ecs_tilemap::{
    prelude::TilemapGridSize,
    tiles::{TilePos, TileStorage, TileTexture},
};
use bevy_rapier2d::prelude::*;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_tiled_colliders);
    }
}

fn add_tiled_colliders(
    mut commands: Commands,
    query: Query<(&TilemapGridSize, &TileStorage, &Transform), Added<TilemapGridSize>>,
    tile_query: Query<(&TilePos, &TileTexture)>,
) {
    for (grid_size, tile_storage, transform) in &query {
        for tile in tile_storage.iter() {
            if let Some(tile) = tile {
                let (tile_pos, tile_texture) = tile_query.get(*tile).unwrap();
                if tile_texture.0 == 2 || tile_texture.0 == 3 {
                    let x = tile_pos.x as f32 * grid_size.x as f32 + 8.;
                    let y = tile_pos.y as f32 * grid_size.y as f32 + 8.;
                    commands
                        .spawn_bundle(TransformBundle::from_transform(
                            transform
                                .clone()
                                .mul_transform(Transform::from_xyz(x, y, 0.)),
                        ))
                        .insert(RigidBody::Fixed)
                        .insert(Restitution {
                            coefficient: 1.,
                            combine_rule: CoefficientCombineRule::Multiply,
                        })
                        .insert(Collider::cuboid(9., 1.));
                }
            }
        }
    }
}
