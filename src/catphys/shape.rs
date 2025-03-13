use crate::vec2::Vec2;

pub enum Shape {
    Circle { radius: f32 },
    Rectangle { upper: Vec2, lower: Vec2 },
    Polygon { verts: Vec<Vec2> },
}

//https://phys.libretexts.org/Bookshelves/College_Physics/College_Physics_1e_(OpenStax)/10%3A_Rotational_Motion_and_Angular_Momentum/10.03%3A_Dynamics_of_Rotational_Motion_-_Rotational_Inertia
impl Shape {
    pub fn rotational_inertia(&self) -> f32 {
        match self {
            Shape::Circle { radius } => 0.5 * radius * radius,
            Shape::Rectangle { upper, lower } => 1.0 / 12.0 * (*upper - *lower).magnitude2(),
            _ => 1.0,
        }
    }
}
