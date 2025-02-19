use crate::entity::{Component, Entity, EntityBuilder};
use crate::physics::Physics;
use crate::vec2::Vec2;

pub const PIXELS_PER_METER: f32 = 10.0;

pub struct World {
    upper_left: Vec2,
    lower_right: Vec2,
    pub physics_components: Vec<Physics>,
    //render_components: Vec<Render>,
    pub entities: Vec<Entity>,
    pub player_input_forces: Vec<Vec2>,
    player_entity_idx: Option<usize>,
}

impl World {
    pub fn new(ul: Vec2, lr: Vec2) -> Self {
        Self {
            upper_left: ul,
            lower_right: lr,
            physics_components: Vec::<Physics>::default(),
            //render_components: Vec::<Render>::default(),
            entities: Vec::<Entity>::default(),
            player_input_forces: Vec::<Vec2>::default(),
            player_entity_idx: None,
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

    pub fn player_add_input_force(&mut self, force: Vec2) {
        self.player_input_forces.push(force);
    }

    pub fn set_player_entity(&mut self, pos: Vec2, mass: f32) {
        let phys_idx = self.add_physics(Physics::new(pos, mass));
        self.player_entity_idx = Some(
            self.add_entity(
                EntityBuilder::default()
                    .with_physics_component(phys_idx)
                    .build(),
            ),
        );
    }

    pub fn player_update_position(&mut self, step: Vec2) {
        if let Some(player_idx) = self.player_entity_idx {
            if let Some(phys_idx) = self.entities[player_idx].get_index_for(Component::Physics) {
                self.physics_components[phys_idx].position += step;
            }
        }
    }

    pub fn update_physics(&mut self, delta_time_seconds: f32) {
        // TODO:
        let gravity = Vec2::new(0.0, 98.1) * PIXELS_PER_METER;
        let force = Vec2::new(2.0, 0.0) * PIXELS_PER_METER;

        // TODO: Controlable entity stuff.
        if let Some(player_idx) = self.player_entity_idx {
            if let Some(phys_idx) = self.entities[player_idx].get_index_for(Component::Physics) {
                for force in &self.player_input_forces {
                    self.physics_components[phys_idx].apply_force(*force);
                }
            }
        }
        self.player_input_forces.clear();

        // TODO:
        self.physics_components.iter_mut().for_each(|physics| {
            let weight = gravity * physics.mass;
            physics.apply_force(weight);
            physics.apply_force(force);
            physics.integrate(delta_time_seconds);
            // TODO:
            if physics.velocity.y < -30.0 {
                physics.velocity.y = -30.0;
            }
            // TODO:
            if physics.position.x <= self.upper_left.x {
                physics.position.x = self.upper_left.x;
                physics.velocity.x = 0.0;
            } else if physics.position.x >= self.lower_right.x {
                physics.position.x = self.lower_right.x;
                physics.velocity.x = 0.0;
            }

            if physics.position.y <= self.upper_left.y {
                physics.position.y = self.upper_left.y;
                physics.velocity.y = 0.0;
            } else if physics.position.y >= self.lower_right.y {
                physics.position.y = self.lower_right.y;
                physics.velocity.y = 0.0;
            }
        });
    }
}
