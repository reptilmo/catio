extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::surface::Surface;
use std::time::Duration;

use catiolib::entity::{Component, EntityBuilder};
use catiolib::graphics::Graphics;
use catiolib::input::Input;
use catiolib::physics::{Body, Physics};
use catiolib::system::System;
use catiolib::vec2::Vec2;
use catiolib::world::{World, PIXELS_PER_METER};

const WIDTH: u32 = 1200u32;
const HEIGHT: u32 = 800u32;

fn make_world() -> World {
    let mut world = World::new(
        Vec2::new(10.0, 10.0),
        Vec2::new((WIDTH - 10) as f32, (HEIGHT - 10) as f32),
    );

    let bottom = (HEIGHT - 10) as f32;
    // TODO:
    let idx = world.add_physics(Physics::new(
        Body::make_circle(10.0),
        Vec2::new(10.0, bottom - 20.0),
        1.0,
    ));
    world.add_entity(EntityBuilder::default().with_physics_component(idx).build());
    let idx = world.add_physics(Physics::new(
        Body::Circle { radius: 10.0 },
        Vec2::new(50.0, bottom - 20.0),
        5.0,
    ));
    world.add_entity(EntityBuilder::default().with_physics_component(idx).build());
    world.set_player_entity(Vec2::new(100.0, bottom), 2.0);

    world
}

fn update_world(
    world: &mut World,
    input: &mut Input,
    gfx: &mut Graphics,
    fps: &Surface,
    delta_time_secs: f32,
) -> bool {
    let mut still_running = true;
    input.update();
    if input.key_pressed(Scancode::Escape) {
        still_running = false;
    }

    if input.key_pressed(Scancode::Space)
    /*&& !input.key_was_pressed(Scancode::Space)*/
    {
        world.player_impulse(Vec2::new(0.0, -100.0) * PIXELS_PER_METER);

        //if input.key_was_pressed(Scancode::Space) {
        //    world.player_update_position(
        //        Vec2::new(0.0, -30.0) * PIXELS_PER_METER * delta_time_secs);
        //}
    }

    if input.key_pressed(Scancode::Right) {
        world.player_impulse(Vec2::new(0.5, 0.0) * PIXELS_PER_METER);
    }

    if input.key_pressed(Scancode::Left) {
        world.player_impulse(Vec2::new(-0.5, 0.0) * PIXELS_PER_METER);
    }

    world.update_physics(delta_time_secs);

    gfx.begin_frame();
    gfx.set_draw_color(255, 0, 0);
    for entity in world.entities.iter() {
        if let Some(idx) = entity.get_index_for(Component::Physics) {
            let pos = world.physics_components[idx].position;
            gfx.draw_circle((pos.x as i32, pos.y as i32), 10);
        }
    }
    gfx.copy_from_surface(fps);
    gfx.end_frame();
    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    still_running
}

fn main() {
    let error = System::init("fonts/WorkSans-Regular.ttf".to_string());
    let mut system = match error {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(ctx) => ctx,
    };

    let error = system.init_graphics(WIDTH, HEIGHT, false);
    let mut graphics = match error {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(gfx) => gfx,
    };

    let error = system.init_input();
    let mut input = match error {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        Ok(inpt) => inpt,
    };

    let mut world = make_world();
    system.run(update_world, &mut world, &mut input, &mut graphics);
}
