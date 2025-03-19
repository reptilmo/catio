extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::surface::Surface;
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
use crate::world::{World, PIXELS_PER_METER};
use catphys::{Shape, Vec2};

const WIDTH: u32 = 1200u32;
const HEIGHT: u32 = 800u32;
const MARGIN: u32 = 25u32;

fn make_world() -> World {
    World::new(
        Vec2::new(MARGIN as f32, MARGIN as f32),
        Vec2::new((WIDTH - MARGIN) as f32, (HEIGHT - MARGIN) as f32),
    )
}

fn update_world(
    world: &mut World,
    input: &mut Input,
    gfx: &mut Graphics,
    texture: &Texture,
    fps: &Surface,
    delta_time_secs: f32,
) -> bool {
    let mut still_running = true;
    input.update();
    if input.key_pressed(Scancode::Escape) {
        still_running = false;
    }

    if input.key_pressed(Scancode::P)
        && !input.key_was_pressed(Scancode::P)
        && world.player_entity_idx.is_none()
    {
        world.spawn_player(((WIDTH / 2) as i32, (HEIGHT / 2) as i32), 0.5, 0.5, 2.0);
    }

    if input.key_pressed(Scancode::Space) && !input.key_was_pressed(Scancode::Space) {
        // TODO: Currently the thing basically flies if you keep pressing space.
        if let Some(player_idx) = world.player_entity_idx {
            let player_phys_idx = world.entities[player_idx].get_index_for(Component::Physics);
            if let Some(idx) = player_phys_idx {
                world.physics_components[idx].position.y -= 0.5 * PIXELS_PER_METER;
            }
        }
    }

    if input.key_pressed(Scancode::Right) {
        if let Some(player_idx) = world.player_entity_idx {
            let player_phys_idx = world.entities[player_idx].get_index_for(Component::Physics);
            if let Some(idx) = player_phys_idx {
                world.flip_player_texture = false;
                world.physics_components[idx]
                    .apply_impulse(Vec2::new(0.08 * PIXELS_PER_METER, 0.0));
            }
        }
    }

    if input.key_pressed(Scancode::Left) {
        if let Some(player_idx) = world.player_entity_idx {
            let player_phys_idx = world.entities[player_idx].get_index_for(Component::Physics);
            if let Some(idx) = player_phys_idx {
                world.flip_player_texture = true;
                world.physics_components[idx]
                    .apply_impulse(Vec2::new(-0.08 * PIXELS_PER_METER, 0.0));
            }
        }
    }

    if input.mouse_pressed(MouseButton::Right) && !input.mouse_was_pressed(MouseButton::Right) {
        world.spawn_ball(input.mouse_position(), 0.05, 50.0);
    } else if input.mouse_pressed(MouseButton::Left) && !input.mouse_was_pressed(MouseButton::Left)
    {
        world.spawn_ball(input.mouse_position(), 0.4, 400.0);
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
                        gfx.set_draw_color(Color::RGB(99, 125, 10));
                    } else {
                        gfx.set_draw_color(color);
                    }
                    match shape {
                        Shape::Circle { radius } => {
                            gfx.draw_circle((pos.x as i32, pos.y as i32), *radius as i32)
                        }
                        Shape::Rect { w, h } => gfx.draw_texture(
                            texture,
                            Rect::new(
                                (w * -0.5 + pos.x) as i32,
                                (h * -0.5 + pos.y) as i32,
                                *w as u32,
                                *h as u32,
                            ),
                            world.flip_player_texture,
                            rotation,
                        ),
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = System::init("fonts/WorkSans-Regular.ttf".to_string())?;
    let mut graphics = system.init_graphics(WIDTH, HEIGHT, false)?;
    let mut input = system.init_input()?;
    let mut world = make_world();
    system.run(update_world, &mut world, &mut input, &mut graphics);
    Ok(())
}
