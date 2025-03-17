use crate::physics::Physics;
use crate::shape::Shape;
use crate::vec2::Vec2;

pub struct Collision {
    normal: Vec2,
    depth: f32,
}

impl Collision {
    pub fn detect(sa: &Shape, sb: &Shape, pa: &Physics, pb: &Physics) -> Option<Collision> {
        match sa {
            Shape::Circle { radius: ra } => match sb {
                Shape::Circle { radius: rb } => {
                    let mut distance = pb.position - pa.position;
                    if distance.magnitude2() < (ra + rb) * (ra + rb) {
                        distance = distance.unit();
                        let start = pb.position - (distance * *rb);
                        let end = pa.position + (distance * *ra);
                        Some(Collision {
                            normal: distance,
                            depth: (start - end).magnitude(),
                        })
                    } else {
                        None
                    }
                }
                Shape::Rect {
                    w: width,
                    h: height,
                } => {
                    let x_min = pb.position.x - (width * 0.5);
                    let x_max = pb.position.x + (width * 0.5);
                    let y_min = pb.position.y - (height * 0.5);
                    let y_max = pb.position.y + (height * 0.5);
                    let p = Vec2::new(
                        pa.position.x.clamp(x_min, x_max),
                        pa.position.y.clamp(y_min, y_max),
                    );

                    let direction = pa.position - p;
                    if direction.magnitude2() <= (ra * ra) {
                        let start = pb.position - (direction * *height);
                        let end = pa.position + (direction * *ra);
                        Some(Collision {
                            normal: direction.unit(),
                            depth: (start - end).magnitude(),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Shape::Rect {
                w: width,
                h: height,
            } => match sb {
                Shape::Circle { radius: rb } => {
                    let x_min = pa.position.x - (width * 0.5);
                    let x_max = pa.position.x + (width * 0.5);
                    let y_min = pa.position.y - (height * 0.5);
                    let y_max = pa.position.y + (height * 0.5);
                    let p = Vec2::new(
                        pb.position.x.clamp(x_min, x_max),
                        pb.position.y.clamp(y_min, y_max),
                    );

                    let direction = pb.position - p;
                    let distance = direction.magnitude2();
                    if distance <= (rb * rb) {
                        Some(Collision {
                            normal: direction.unit(),
                            depth: rb - distance,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub fn resolve_penetration(&self, inverse_mass_a: f32, inverse_mass_b: f32) -> (Vec2, Vec2) {
        let d = self.depth / (inverse_mass_a + inverse_mass_b);
        (
            self.normal * d * inverse_mass_a,
            self.normal * d * inverse_mass_b,
        )
    }

    pub fn resolve_impulse(&self, pa: &Physics, pb: &Physics) -> Vec2 {
        let e = f32::min(pa.restitution, pb.restitution); // Collision ellasticity
        let v = pa.velocity - pb.velocity; // Relative velocity
        let impulse_magnitude =
            -(1.0 + e) * self.normal.dot(v) / (pa.inverse_mass + pb.inverse_mass);

        self.normal * impulse_magnitude
    }
}
