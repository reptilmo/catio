use crate::physics::Physics;
use crate::shape::Shape;
use crate::vec2::Vec2;

pub struct Collision {
    normal: Vec2,
    depth: f32,
}

impl Collision {
    // Is there a better way to implement this double match?
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

    //TODO: This is a hacky function. The issue comes from the coordinate system with the
    //origin in the upper left corner of the screen/window. This makes working with vectors,
    //specifically finding the right normal vector, stupid. I should switch to world origin in the
    //center of the screen later when I convert rendering to OpneGL or another GPU accelerated API.
    fn rect_circle(
        width: f32,
        height: f32,
        radius: f32,
        pr: &Physics,
        pc: &Physics,
    ) -> Option<Collision> {
        let vert = [
            Vec2::new(width * -0.5, height * -0.5) + pr.position,
            Vec2::new(width * 0.5, height * -0.5) + pr.position,
            Vec2::new(width * 0.5, height * 0.5) + pr.position,
            Vec2::new(width * -0.5, height * 0.5) + pr.position,
        ];

        let mut outside = false;
        let mut distance_to_edge = f32::MIN; // From circle position.
        let mut v0 = Vec2::default();
        let mut v1 = Vec2::default();

        for p0 in 0..4 {
            let p1 = (p0 + 1) % 4; //TODO: Avoid this division.

            v0 = vert[p0];
            v1 = vert[p1];

            let normal = (v1 - v0).normal_positive().unit();
            let projection = (pc.position - v0).dot(normal);

            if projection > 0.0 {
                distance_to_edge = projection;
                outside = true;
                break;
            }

            if projection > distance_to_edge {
                distance_to_edge = projection;
            }
        }

        if outside {
            let mut edge = v1 - v0;
            let mut intersect = pc.position - v0;
            if intersect.dot(edge) < 0.0 {
                if intersect.magnitude2() <= radius * radius {
                    return Some(Collision {
                        normal: -intersect.unit(),
                        depth: radius - intersect.magnitude(),
                    });
                }
            } else {
                edge = v0 - v1;
                intersect = pc.position - v1;
                if intersect.dot(edge) < 0.0 {
                    if intersect.magnitude2() <= radius * radius {
                        return Some(Collision {
                            normal: -intersect.unit(),
                            depth: radius - intersect.magnitude(),
                        });
                    }
                } else if distance_to_edge <= radius {
                    return Some(Collision {
                        normal: (v0 - v1).normal_positive().unit(),
                        depth: radius - distance_to_edge,
                    });
                }
            }
        } else {
            // Inside
            return Some(Collision {
                normal: (v0 - v1).normal_positive().unit(),
                depth: radius - distance_to_edge,
            });
        }

        None
    }
}
