use crate::vec2::Vec2;

pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl Physics {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            mass,
        }
    }
}
