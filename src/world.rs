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
        self.physics_components.iter_mut().for_each(|physics| {
            physics.position += physics.velocity * delta_time_seconds;
        });
    }

    pub fn for_each_entity<F>(&self, process_entity: F)
    where
        F: Fn(&Entity),
    {
        self.entities.iter().for_each(process_entity);
    }
}
