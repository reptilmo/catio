use crate::entity::Entity;
use crate::physics::Physics;
use crate::vec2::Vec2;

const PIXELS_PER_METER: f32 = 10.0;

pub struct World {
    upper_left: Vec2,
    lower_right: Vec2,
    pub physics_components: Vec<Physics>,
    //render_components: Vec<Render>,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new(ul: Vec2, lr: Vec2) -> Self {
        Self {
            upper_left: ul,
            lower_right: lr,
            physics_components: Vec::<Physics>::default(),
            //render_components: Vec::<Render>::default(),
            entities: Vec::<Entity>::default(),
        }
    }

    pub fn add_physics(&mut self, phys: Physics) -> usize {
        self.physics_components.push(phys);
        self.physics_components.len() - 1
    }

    pub fn add_entity(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn update_physics(&mut self, delta_time_seconds: f32) {
        // TODO:
        let gravity = Vec2::new(0.0, 9.81) * PIXELS_PER_METER;
        let force = Vec2::new(2.0, 0.0) * PIXELS_PER_METER;
        // TODO:
        self.physics_components.iter_mut().for_each(|physics| {
            let weight = gravity * physics.mass;
            physics.apply_force(weight);
            physics.apply_force(force);
            physics.integrate(delta_time_seconds);
            // TODO:
            if physics.position.x <= self.upper_left.x {
                physics.position.x = self.upper_left.x;
                physics.velocity.x *= -1.0;
            } else if physics.position.x >= self.lower_right.x {
                physics.position.x = self.lower_right.x;
                physics.velocity.x *= -1.0;
            }

            if physics.position.y <= self.upper_left.y {
                physics.position.y = self.upper_left.y;
                physics.velocity.y *= -1.0;
            } else if physics.position.y >= self.lower_right.y {
                physics.position.y = self.lower_right.y;
                physics.velocity.y *= -1.0;
            }
        });
    }

    pub fn for_each_entity<F>(&self, process_entity: F)
    where
        F: Fn(&Entity),
    {
        self.entities.iter().for_each(process_entity);
    }
}
