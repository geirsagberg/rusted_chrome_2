use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraPlugin, PixelViewport, PixelZoom};

use crate::world::GameWorld;

pub struct CameraPlugin;

#[derive(Component)]
pub struct CameraTarget {
    pub radius: f32,
}

impl CameraTarget {
    pub fn with_radius(radius: f32) -> Self {
        Self { radius }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, follow_targets);
    }
}
fn setup_camera(mut commands: Commands, game_world: Res<GameWorld>) {
    // camera.transform.translation.x = game_world.width / 2.;
    // camera.transform.translation.y = game_world.height / 2.;

    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.transform.translation.x = game_world.width / 2.;
    camera2d_bundle.transform.translation.y = game_world.height / 2.;
    commands.spawn((camera2d_bundle, PixelZoom::Fixed(2), PixelViewport));
}

fn follow_targets(
    query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<
        (&mut Transform, &PixelViewport),
        (With<Camera>, Without<CameraTarget>),
    >,
    time: Res<Time>,
    game_world: Res<GameWorld>,
) {
    let (mut camera, pixel_viewport) = camera_query.single_mut();

    // find max and min x and y of targets
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for transform in query.iter() {
        let position = transform.translation;
        if position.x < min_x {
            min_x = position.x;
        }
        if position.x > max_x {
            max_x = position.x;
        }
        if position.y < min_y {
            min_y = position.y;
        }
        if position.y > max_y {
            max_y = position.y;
        }
    }

    // find center of all transforms
    let center_x = (min_x + max_x) / 2.0;
    let center_y = (min_y + max_y) / 2.0;

    let slack = 50.;

    let follow_speed = 1.5;

    if (camera.translation.x - center_x).abs() > slack {
        let delta = (center_x - camera.translation.x) * time.delta_seconds() * follow_speed;
        let new_x = camera.translation.x + delta;
        // let new_x = new_x.clamp(
        //     -pixel_viewport.left,
        //     game_world.width - pixel_viewport.right,
        // );
        camera.translation.x = new_x;
    }

    if (camera.translation.y - center_y).abs() > slack {
        let delta = (center_y - camera.translation.y) * time.delta_seconds() * follow_speed;
        let new_y = camera.translation.y + delta;
        // let new_y = new_y.clamp(
        //     -pixel_viewport.bottom,
        //     game_world.height - pixel_viewport.top,
        // );
        camera.translation.y = new_y;
    }
}
