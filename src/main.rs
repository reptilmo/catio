extern crate sdl2;

use sdl2::keyboard::Scancode;
use sdl2::timer;

use std::time::Duration;

use catiolib::graphics::Graphics;
use catiolib::input::Input;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let mut graphics = Graphics::create(&video, 800u32, 600u32, false);
    let mut input = Input::default();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        event_pump.pump_events();
        input.update(&event_pump);
        if input.key_pressed(Scancode::Escape) {
            running = false;
        }

        graphics.begin_frame();
        graphics.set_draw_color(255, 0, 0);
        graphics.draw_circle((400, 300), 10);
        graphics.end_frame();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
