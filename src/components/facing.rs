use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, PartialEq, Eq, Clone)]
pub enum Facing {
    Left,
    Right,
}

impl Facing {
    pub fn is_left(&self) -> bool {
        self == &Facing::Left
    }

    pub fn set(&mut self, facing: Facing) {
        *self = facing;
    }
}
