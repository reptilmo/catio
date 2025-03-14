use crate::physics::Physics;
use crate::shape::Shape;

pub struct Collision {}

impl Collision {
    pub fn test(sa: &Shape, sb: &Shape, pa: &Physics, pb: &Physics) -> bool {
        match sa {
            Shape::Circle { radius: ra } => match sb {
                Shape::Circle { radius: rb } => {
                    (pb.position - pa.position).magnitude2() <= (ra + rb) * (ra + rb)
                },
                _ => false,
            },
            _ => false,
        }
    }
}
