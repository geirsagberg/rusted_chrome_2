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
    let mut camera2d_bundle = Camera2dBundle::default();
    camera2d_bundle.transform.translation.x = game_world.width;
    camera2d_bundle.transform.translation.y = game_world.height;
    commands.spawn((camera2d_bundle, PixelZoom::Fixed(2), PixelViewport));
}

fn follow_targets(
    query: Query<&Transform, With<CameraTarget>>,
    mut camera_query: Query<(&mut Transform, &Camera, &PixelZoom), (Without<CameraTarget>)>,
    time: Res<Time>,
    game_world: Res<GameWorld>,
) {
    let (mut camera_transform, camera, pixel_zoom) = camera_query.single_mut();

    let viewport = camera.logical_viewport_rect().unwrap();
    let (pixel_width, pixel_height) = match (pixel_zoom) {
        PixelZoom::Fixed(zoom) => {
            let pixel_width = viewport.width() as f32 / *zoom as f32;
            let pixel_height = viewport.height() as f32 / *zoom as f32;
            (pixel_width, pixel_height)
        }
        _ => (0., 0.),
    };

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

    if (camera_transform.translation.x - center_x).abs() > slack {
        let delta =
            (center_x - camera_transform.translation.x) * time.delta_seconds() * follow_speed;
        let new_x = camera_transform.translation.x + delta;
        let new_x = new_x.clamp(pixel_width / 2., game_world.width - pixel_width / 2.);
        camera_transform.translation.x = new_x;
    }

    if (camera_transform.translation.y - center_y).abs() > slack {
        let delta =
            (center_y - camera_transform.translation.y) * time.delta_seconds() * follow_speed;
        let new_y = camera_transform.translation.y + delta;
        let new_y = new_y.clamp(pixel_height / 2., game_world.height - pixel_height / 2.);
        camera_transform.translation.y = new_y;
    }
}
