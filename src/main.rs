extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::surface::Surface;
use std::time::Duration;

mod entity;
mod graphics;
mod input;
mod system;
mod world;

use crate::entity::{Component, EntityBuilder};
use crate::graphics::Graphics;
use crate::input::Input;
use crate::system::System;
use crate::world::{World, PIXELS_PER_METER};
use catphys::{Shape, Vec2};

const WIDTH: u32 = 1200u32;
const HEIGHT: u32 = 800u32;

fn make_world() -> World {
    let world = World::new(
        Vec2::new(10.0, 10.0),
        Vec2::new((WIDTH - 10) as f32, (HEIGHT - 10) as f32),
    );

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

    if input.mouse_pressed(MouseButton::Left) && !input.mouse_was_pressed(MouseButton::Left) {
        world.spawn_ball(input.mouse_position());
    } else if input.mouse_pressed(MouseButton::Right)
        && !input.mouse_was_pressed(MouseButton::Right)
    {
        world.spawn_box(input.mouse_position());
    }

    world.update_physics(delta_time_secs);

    gfx.begin_frame();
    for entity in world.entities.iter() {
        if let Some(idx) = entity.get_index_for(Component::Render) {
            let color = world.render_components[idx].color;
            if let Some(idx) = entity.get_index_for(Component::Physics) {
                let pos = world.physics_components[idx].position;
                let rotation = world.physics_components[idx].rotation;
                if let Some(idx) = entity.get_index_for(Component::Shape) {
                    let shape = &world.shape_components[idx];
                    gfx.set_draw_color(color);
                    match shape {
                        Shape::Circle { radius } => gfx.draw_circle(
                            (pos.x as i32, pos.y as i32),
                            (radius * PIXELS_PER_METER) as i32,
                        ),
                        Shape::Rectangle { w, h } => {
                            gfx.draw_box(pos, w * PIXELS_PER_METER, h * PIXELS_PER_METER, rotation)
                        }
                        _ => (),
                    }
                }
            }
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
