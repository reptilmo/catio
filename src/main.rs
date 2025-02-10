extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::surface::Surface;
use std::time::Duration;

use catiolib::graphics::Graphics;
use catiolib::input::Input;
use catiolib::system::System;

fn frame(input: &mut Input, gfx: &mut Graphics, fps: &Surface, delta_time_secs: f32) -> bool {
    //println!("{}", delta_time_secs);
    let mut still_running = true;
    input.update();
    if input.key_pressed(Scancode::Escape) {
        still_running = false;
    }

    gfx.begin_frame();
    gfx.set_draw_color(255, 0, 0);
    gfx.draw_circle((400, 300), 10);

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

    let error = system.init_graphics(800u32, 600u32, false);
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

    system.run(frame, &mut input, &mut graphics);
}
