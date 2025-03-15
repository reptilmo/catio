use crate::physics::Physics;
use crate::shape::Shape;
use crate::vec2::Vec2;

pub struct Collision {
    normal: Vec2,
    start: Vec2,
    end: Vec2,
}

impl Collision {
    pub fn detect(sa: &Shape, sb: &Shape, pa: &Physics, pb: &Physics) -> Option<Collision> {
        match sa {
            Shape::Circle { radius: ra } => match sb {
                Shape::Circle { radius: rb } => {
                    let mut distance = pb.position - pa.position;
                    if distance.magnitude2() <= (ra + rb) * (ra + rb) {
                        distance = distance.unit();
                        Some(Collision {
                            normal: distance,
                            start: pb.position - (distance * *rb),
                            end: pa.position + (distance * *ra),
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

    pub fn resolve_projection(&self, inverse_mass_a: f32, inverse_mass_b: f32) -> (Vec2, Vec2) {
        let distance = (self.end - self.start).magnitude();
        let d = distance / (inverse_mass_a + inverse_mass_b);
        (
            self.normal * d * inverse_mass_a,
            self.normal * d * inverse_mass_b,
        )
    }

    pub fn resolve_impulse(&self, pa: &Physics, pb: &Physics) -> Vec2 {
        let e = f32::min(pa.restitution, pb.restitution);
        let v = pa.velocity - pb.velocity;
        let impulse_magnitude =
            -(1.0 + e) * self.normal.dot(v) / (pa.inverse_mass + pb.inverse_mass);

        self.normal * impulse_magnitude
    }
}
