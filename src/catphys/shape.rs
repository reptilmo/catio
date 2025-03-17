use crate::vec2::Vec2;

pub enum Shape {
    Circle { radius: f32 },
    Rect { w: f32, h: f32 }, // AABB
    Polygon { verts: Vec<Vec2> },
}

//https://phys.libretexts.org/Bookshelves/College_Physics/
impl Shape {
    pub fn rotational_inertia(&self) -> f32 {
        match self {
            Shape::Circle { radius } => 0.5 * radius * radius,
            Shape::Rect { w, h } => 1.0 / 12.0 * (w * w + h * h),
            _ => 1.0,
        }
    }
}
