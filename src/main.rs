extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::surface::Surface;
use std::path::Path;
//use std::time::Duration;

mod entity;
mod graphics;
mod input;
mod system;
mod world;

use crate::entity::Component;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::system::System;
use crate::world::World;
use catphys::{Shape, Vec2};

const WIDTH: u32 = 1200u32;
const HEIGHT: u32 = 800u32;

fn make_world() -> World {
    World::new(
        Vec2::new(10.0, 10.0),
        Vec2::new((WIDTH - 10) as f32, (HEIGHT - 10) as f32),
    )
}

fn update_world(
    world: &mut World,
    input: &mut Input,
    gfx: &mut Graphics,
    textures: &[Texture],
    fps: &Surface,
    delta_time_secs: f32,
) -> bool {
    let mut still_running = true;
    input.update();
    if input.key_pressed(Scancode::Escape) {
        still_running = false;
    }

    if input.key_pressed(Scancode::P) && !input.key_was_pressed(Scancode::P) && !world.player {
        world.spawn_box(((WIDTH / 2) as i32, (HEIGHT / 2) as i32), 2.0, 2.0, 1.0);
        world.player = true;
    }

    if input.mouse_pressed(MouseButton::Left) && !input.mouse_was_pressed(MouseButton::Left) {
        for _ in 0..10 {
            world.spawn_ball(input.mouse_position(), 0.05, 1.0);
        }
    } else if input.mouse_pressed(MouseButton::Right)
        && !input.mouse_was_pressed(MouseButton::Right)
    {
        world.spawn_ball(input.mouse_position(), 0.5, 10.0);
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
                    if entity.colliding {
                        gfx.set_draw_color(Color::RGB(255, 255, 255));
                    } else {
                        gfx.set_draw_color(color);
                    }
                    match shape {
                        Shape::Circle { radius } => {
                            gfx.draw_circle((pos.x as i32, pos.y as i32), *radius as i32)
                        }
                        Shape::Rect { w, h } => gfx.draw_box(pos, *w, *h, rotation),
                        _ => (),
                    }
                }
            }
        }
    }
    gfx.copy_from_surface(fps);
    gfx.end_frame();
    //std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    still_running
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let system = System::init("fonts/WorkSans-Regular.ttf".to_string())?;
    let mut graphics = system.init_graphics(WIDTH, HEIGHT, false)?;
    let mut input = system.init_input()?;
    let mut world = make_world();
    system.run(update_world, &mut world, &mut input, &mut graphics);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
                   
