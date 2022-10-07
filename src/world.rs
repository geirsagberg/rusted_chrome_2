use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    loading::{TextureAssets, TiledAssets},
    tiled_map::TiledMapBundle,
    GameState,
};

pub struct WorldPlugin;

pub struct GameWorld {
    pub width: f32,
    pub height: f32,
}

const WORLD_WIDTH: f32 = 960.;
const WORLD_HEIGHT: f32 = 540.;

impl Default for GameWorld {
    fn default() -> Self {
        Self {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
        }
    }
}

#[derive(Component)]
pub struct ClampToWorld;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameWorld::default())
            .add_enter_system_set(
                GameState::Playing,
                SystemSet::new()
                    .with_system(create_world)
                    .with_system(create_tiles),
            );
    }
}

fn create_tiles(mut commands: Commands, tiled: Res<TiledAssets>) {
    commands.spawn_bundle(TiledMapBundle {
        tiled_map: tiled.city.clone(),
        ..default()
    });
}

fn create_world(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.background.clone(),
        sprite: Sprite {
            custom_size: Some(Vec2::new(WORLD_WIDTH, WORLD_HEIGHT)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn get_world_rollback_systems() -> SystemSet {
    SystemSet::new().with_system(wrap_around_world)
}

fn wrap_around_world(mut query: Query<(&mut Transform, &Collider)>, world: Res<GameWorld>) {
    for (mut transform, collider) in &mut query {
        let position = transform.translation;
        let size = collider.raw.compute_local_aabb().half_extents();
        let width = size.x * 2.;
        let height = size.y * 2.;

        if position.x > world.width / 2. + width {
            transform.translation.x = -world.width / 2. - width;
        } else if position.x < -world.width / 2. - width {
            transform.translation.x = world.width / 2. + width;
        }

        if position.y > world.height / 2. + height {
            transform.translation.y = -world.height / 2. - height;
        } else if position.y < -world.height / 2. - height {
            transform.translation.y = world.height / 2. + height;
        }
    }
}
