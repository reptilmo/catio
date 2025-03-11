use crate::vec2::Vec2;

pub struct Force {}

impl Force {
    pub fn friction(k: f32, velocity: Vec2) -> Vec2 {
        -velocity * k
    }

    pub fn drag(k: f32, velocity: Vec2) -> Vec2 {
        -velocity * velocity.magnitude2() * k
    }

    pub fn spring(_k: f32, _object_position: Vec2, _anchor_position: Vec2) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
}
