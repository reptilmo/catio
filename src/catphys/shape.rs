use crate::vec2::Vec2;

pub enum Shape {
    Circle { radius: f32 },
    Rectangle { top_left: Vec2, bottom_right: Vec2 },
    Polygon { verts: Vec<Vec2> },
}

impl Shape {
    pub fn make_circle(radius: f32) -> Self {
        Shape::Circle { radius }
    }

    pub fn make_polygon(verts: Vec<Vec2>) -> Self {
        Shape::Polygon { verts }
    }

    pub fn moment_of_inertia(&self) -> f32 {
        match self {
            Shape::Circle { radius } => *radius,
            _ => 0.0,
        }
    }
}

