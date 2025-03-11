use crate::vec2::Vec2;

pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub forces: Vec2,
    pub mass: f32,
    pub inverse_mass: f32,

    pub rotation: f32,
    pub angular_velocity: f32,
    pub torque: f32,
    pub angular_mass: f32,
    pub inverse_angular_mass: f32,

    pub shape_idx: i32,
}

impl Physics {
    pub fn new(position: Vec2, mass: f32) -> Self {
        let angular_mass: f32 = 1.0; // TODO: Depends on the shape.

        Self {
            position,
            velocity: Vec2::new(0.0, 0.0),
            forces: Vec2::new(0.0, 0.0),
            mass,
            inverse_mass: 1.0 / mass,
            rotation: 0.0,
            angular_velocity: 0.0,
            torque: 0.0,
            angular_mass,
            inverse_angular_mass: 1.0 / angular_mass,
            shape_idx: -1,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.forces += force;
    }

    pub fn apply_torque(&mut self, torq: f32) {
        self.torque += torq;
    }

    pub fn apply_impulse(&mut self, impulse: Vec2) {
        self.velocity += impulse * self.inverse_mass;
    }

    pub fn integrate(&mut self, dt: f32) {
        let acceleration = self.forces * self.inverse_mass;
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;

        self.forces = Vec2::new(0.0, 0.0);
    }

    pub fn integrate_angular(&mut self, dt: f32) {
        let angular_acc = self.torque * self.inverse_angular_mass;
        self.angular_velocity += angular_acc * dt;
        self.rotation += self.angular_velocity * dt;

        self.torque = 0.0;
    }
}
