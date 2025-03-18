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
                } => Self::rect_circle(*width, *height, *ra, pb, pa),
                _ => None,
            },
            Shape::Rect {
                w: width,
                h: height,
            } => match sb {
                Shape::Circle { radius: r } => Self::rect_circle(*width, *height, *r, pb, pa),
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

    fn rect_circle(
        width: f32,
        height: f32,
        _radius: f32,
        pr: &Physics,
        pc: &Physics,
    ) -> Option<Collision> {
        let vert = [
            Vec2::new(width * -0.5, height * -0.5) + pr.position,
            Vec2::new(width * 0.5, height * -0.5) + pr.position,
            Vec2::new(width * 0.5, height * 0.5) + pr.position,
            Vec2::new(width * -0.5, height * 0.5) + pr.position,
        ];

        for p0 in 0..4 {
            let p1 = (p0 + 1) % 4; //TODO: Avoid this division.

            let normal = (vert[p1] - vert[p0]).normal_positive().unit();
            let projection = (pc.position - vert[p0]).dot(normal);

            if projection > 0.0 {
                println!("Closest edge: {:?} {:?}", vert[p0], vert[p1]); //TODO:
            }
        }

        None
    }
}
