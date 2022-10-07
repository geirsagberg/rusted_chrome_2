use bevy::{prelude::Component, reflect::Reflect};

#[derive(Component, PartialEq, Eq, Clone, Debug, Reflect)]
pub enum Facing {
    Left,
    Right,
}

impl Default for Facing {
    fn default() -> Self {
        Self::Right
    }
}

impl Facing {
    pub fn is_left(&self) -> bool {
        self == &Facing::Left
    }

    pub fn set(&mut self, facing: Facing) {
        *self = facing;
    }
}
