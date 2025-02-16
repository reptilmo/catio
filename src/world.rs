use crate::entity::Entity;
use crate::physics::Physics;

pub struct World {
    pub physics_components: Vec<Physics>,
    //render_components: Vec<Render>,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            physics_components: Vec::<Physics>::default(),
            //render_components: Vec::<Render>::default(),
            entities: Vec::<Entity>::default(),
        }
    }

    pub fn update_physics(&mut self, delta_time_seconds: f32) {
        // TODO:
        self.physics_components.iter_mut().for_each(|physics| {
            physics.position += physics.velocity * delta_time_seconds;
        });
    }
}
