extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::timer;
use std::time::Duration;

use catiolib::input::Input;

fn main() {
    catiolib::test(); // call something from the lib
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("catio", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut input = Input::default();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    while running {
        event_pump.pump_events();
        input.update(&event_pump);
        if input.key_pressed(Scancode::Escape) {
            running = false;
        }

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
        canvas.present();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
