use crate::vec2::Vec2;

pub enum Body {
    Circle { radius: f32 },
    Polygon { verts: Vec<Vec2> },
}

impl Body {
    pub fn make_circle(radius: f32) -> Self {
        Body::Circle { radius }
    }

    pub fn make_polygon(verts: Vec<Vec2>) -> Self {
        Body::Polygon { verts }
    }

    pub fn moment(&self) -> f32 {
        match self {
            Body::Circle { radius } => *radius,
            _ => 0.0,
        }
    }
}

pub struct Physics {
    pub position: Vec2,
    pub velocity: Vec2,
    pub forces: Vec2,
    pub inverse_mass: f32,
    pub body: Body,
}

pub struct Force {}

impl Force {
    pub fn friction(k: f32, velocity: Vec2) -> Vec2 {
        -velocity * k
    }

    pub fn drag(k: f32, velocity: Vec2) -> Vec2 {
        -velocity * velocity.magnitude2() * k
    }

    pub fn spring() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
}

impl Physics {
    pub fn new(body: Body, position: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec2::new(0.0, 0.0),
            forces: Vec2::new(0.0, 0.0),
            inverse_mass: 1.0 / mass,
            body,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.forces += force;
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
}
