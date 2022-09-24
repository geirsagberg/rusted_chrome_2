use std::f32::consts::PI;

use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Aiming {
    pub angle: f32,
    pub max_angle: f32,
    pub min_angle: f32,
}

#[derive(Component, PartialEq, Eq, Clone, Debug)]
pub struct AimingChild;

impl Default for Aiming {
    fn default() -> Self {
        Self {
            angle: 0.0,
            max_angle: PI / 2.0,
            min_angle: -PI / 2.0,
        }
    }
}
