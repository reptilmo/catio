use crate::entity::{Component, Entity, EntityBuilder};
use crate::graphics::Render;
use catphys::{Force, Physics, Shape, Vec2};
use sdl2::pixels::Color;

pub const PIXELS_PER_METER: f32 = 100.0;

pub struct World {
    upper_left: Vec2,
    lower_right: Vec2,
    pub physics_components: Vec<Physics>,
    pub shape_components: Vec<Shape>,
    pub render_components: Vec<Render>,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new(ul: Vec2, lr: Vec2) -> Self {
        Self {
            upper_left: ul,
            lower_right: lr,
            physics_components: Vec::<Physics>::default(),
            shape_components: Vec::<Shape>::default(),
            render_components: Vec::<Render>::default(),
            entities: Vec::<Entity>::default(),
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

    pub fn spawn_ball(&mut self, pos: (i32, i32)) {
        let render = Render {
            color: Color::RGB(255, 0, 0),
            fill: true,
        };
        let rend_idx = self.add_render(render); //TODO: Only need one render component if the balls
                                                //all look the same.
        let ball = Shape::Circle { radius: 0.1 };
        let phys_idx = self.add_physics(Physics::new(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            5.0,
            ball.rotational_inertia(),
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

    pub fn spawn_box(&mut self, pos: (i32, i32)) {
        let render = Render {
            color: Color::RGB(50, 50, 255),
            fill: false,
        };
        let rend_idx = self.add_render(render); //TODO: As above.
        let rect = Shape::Rectangle { w: 0.2, h: 0.2 };
        let phys_idx = self.add_physics(Physics::new(
            Vec2::new(pos.0 as f32, pos.1 as f32),
            10.0,
            rect.rotational_inertia(),
        ));
        let shape_idx = self.add_shape(rect);
        self.add_entity(
            EntityBuilder::default()
                .with_shape_component(shape_idx)
                .with_physics_component(phys_idx)
                .with_render_component(rend_idx)
                .build(),
        );
    }

    pub fn update_physics(&mut self, delta_time_seconds: f32) {
        // TODO:
        let gravity = Vec2::new(0.0, 9.81) * PIXELS_PER_METER;
        let _wind = Vec2::new(5.0, 0.0) * PIXELS_PER_METER;
        let torque = 0.001;

        // TODO:
        self.physics_components.iter_mut().for_each(|physics| {
            let weight = gravity / physics.inverse_mass;
            physics.apply_force(weight);
            physics.apply_torque(torque);
            physics.apply_force(Force::drag(0.001, physics.velocity));
            //physics.apply_force(Force::friction(0.65, physics.velocity));
            //physics.apply_force(wind);
            physics.integrate(delta_time_seconds);
            physics.integrate_angular(delta_time_seconds);
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
                physics.velocity.y *= -0.7;
            }
        });
    }
}
