use crate::vec2::Vec2;

pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub forces: Vec2,
    pub mass: f32, // TODO: only need inverse?
}

impl Physics {
    pub fn new(position: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec2::new(0.0, 0.0),
            forces: Vec2::new(0.0, 0.0),
            mass,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.forces += force;
    }

    pub fn integrate(&mut self, dt: f32) {
        let acceleration = self.forces / self.mass;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        self.forces = Vec2::new(0.0, 0.0);
    }
}
