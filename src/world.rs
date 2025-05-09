use crate::entity::{Component, Entity, EntityBuilder};
use crate::graphics::Render;
use catphys::{Collision, Force, Physics, Shape, Vec2};
use sdl2::pixels::Color;

pub const PIXELS_PER_METER: f32 = 100.0;

pub struct World {
    upper_left: Vec2,
    lower_right: Vec2,

    // Environment and world forces.
    gravitational_acceleration: Vec2,

    // Entity-component.
    pub physics_components: Vec<Physics>,
    pub shape_components: Vec<Shape>,
    pub render_components: Vec<Render>,
    pub entities: Vec<Entity>,
    pub player_entity_idx: Option<usize>,
    pub flip_player_texture: bool,
}

impl World {
    pub fn new(ul: Vec2, lr: Vec2) -> Self {
        Self {
            upper_left: ul,
            lower_right: lr,
            gravitational_acceleration: Vec2::new(0.0, 9.81) * PIXELS_PER_METER,
            physics_components: Vec::<Physics>::default(),
            shape_components: Vec::<Shape>::default(),
            render_components: Vec::<Render>::default(),
            entities: Vec::<Entity>::default(),
            player_entity_idx: None,
            flip_player_texture: false,
        }
    }

    pub fn add_shape(&mut self, shape: Shape) -> usize {
        self.shape_components.push(shape);
        self.shape_components.len() - 1
    }

    pub fn add_physics(&mut self, phys: Physics) -> usize {
        self.physics_components.push(phys);
        self.physics_components.len() - 1
    }

    pub fn add_render(&mut self, rend: Render) -> usize {
        self.render_components.push(rend);
        self.render_components.len() - 1
    }

    pub fn add_entity(&mut self, entity: Entity) -> usize {
        self.entities.push(entity);
        self.entities.len() - 1
    }

    pub fn spawn_ball(&mut self, pos: (i32, i32), rad: f32, mass: f32) {
        let render = Render {
            color: Color::RGB(255, 255, 255),
        };
        //TODO: Only need one render component
        // if the balls all look the same.
        let rend_idx = self.add_render(render);
        let ball = Shape::Circle {
            radius: rad * PIXELS_PER_METER,
        };
        let phys_idx = self.add_physics(Physics::new(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            mass,
            ball.rotational_inertia(),
            0.5,
        ));
        let shape_idx = self.add_shape(ball);
        self.add_entity(
            EntityBuilder::default()
                .with_shape_component(shape_idx)
                .with_physics_component(phys_idx)
                .with_render_component(rend_idx)
                .build(),
        );
    }

    pub fn spawn_player(&mut self, pos: (i32, i32), width: f32, height: f32, mass: f32) {
        let render = Render {
            color: Color::RGB(0, 255, 0),
        };
        let rend_idx = self.add_render(render);
        let rect = Shape::Rect {
            w: width * PIXELS_PER_METER,
            h: height * PIXELS_PER_METER,
        };
        let phys_idx = self.add_physics(Physics::new(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            mass,
            rect.rotational_inertia(),
            0.2,
        ));
        let shape_idx = self.add_shape(rect);
        self.player_entity_idx = Some(
            self.add_entity(
                EntityBuilder::default()
                    .with_shape_component(shape_idx)
                    .with_physics_component(phys_idx)
                    .with_render_component(rend_idx)
                    .build(),
            ),
        );
    }

    pub fn update_physics(&mut self, delta_time_seconds: f32) {
        self.physics_components.iter_mut().for_each(|physics| {
            let weight = self.gravitational_acceleration * physics.mass;
            physics.apply_force(weight);
            //physics.apply_torque(0.01);
            //physics.apply_force(Force::drag(0.001, physics.velocity));
            physics.apply_force(Force::friction(0.65, physics.velocity));
            physics.integrate(delta_time_seconds);
            physics.integrate_angular(delta_time_seconds);
        });

        for entity in &mut self.entities {
            entity.colliding = false;
        }

        // TODO: Eventually, instead of looping over all the entities, will use some sort of
        // quadtree. Doing so would allow parallelizing this part.
        if self.entities.len() > 1 {
            for i in 0..self.entities.len() - 1 {
                let Some(si) = self.entities[i].get_index_for(Component::Shape) else {
                    continue;
                };
                let Some(pi) = self.entities[i].get_index_for(Component::Physics) else {
                    continue;
                };

                for j in i + 1..self.entities.len() {
                    let Some(sj) = self.entities[j].get_index_for(Component::Shape) else {
                        break;
                    };
                    let Some(pj) = self.entities[j].get_index_for(Component::Physics) else {
                        break;
                    };

                    if let Some(collision) = Collision::detect(
                        &self.shape_components[si],
                        &self.shape_components[sj],
                        &self.physics_components[pi],
                        &self.physics_components[pj],
                    ) {
                        self.entities[j].colliding = true;
                        self.entities[i].colliding = true;

                        let displacement = collision.resolve_penetration(
                            self.physics_components[pi].inverse_mass,
                            self.physics_components[pj].inverse_mass,
                        );

                        self.physics_components[pi].position -= displacement.0;
                        self.physics_components[pj].position += displacement.1;

                        let impulse = collision.resolve_impulse(
                            &self.physics_components[pi],
                            &self.physics_components[pj],
                        );

                        self.physics_components[pi].apply_impulse(impulse);
                        self.physics_components[pj].apply_impulse(-impulse);
                    }
                }
            }
        }
        // TODO: This is a very hacky way to keep objects on screen.
        self.physics_components.iter_mut().for_each(|physics| {
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
