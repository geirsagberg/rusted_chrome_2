use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

pub struct World {
    pub width: f32,
    pub height: f32,
}

impl Default for World {
    fn default() -> Self {
        Self {
            width: 800.,
            height: 600.,
        }
    }
}

#[derive(Component)]
pub struct ClampToWorld;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(World::default());
    }
}

pub fn get_world_rollback_systems() -> SystemSet {
    SystemSet::new().with_system(wrap_around_world)
    // .with_system(clamp_to_world)
}

fn wrap_around_world(mut query: Query<(&mut Transform, &Collider)>, world: Res<World>) {
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
