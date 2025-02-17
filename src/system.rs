extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::{Sdl, VideoSubsystem};
use std::path::Path;
use std::time::SystemTime;

use crate::graphics::Graphics;
use crate::input::Input;
use crate::world::World;

pub struct System {
    sdl_context: Sdl,
    ttf_context: Sdl2TtfContext,
    video_subsystem: VideoSubsystem,
    system_font_path: String,
}

impl System {
    pub fn init(font_path: String) -> Result<Self, String> {
        let result = sdl2::init();
        let context = match result {
            Ok(sdl) => sdl,
            Err(e) => return Err(e),
        };

        let result = sdl2::ttf::init();
        let ttf = match result {
            Ok(ctx) => ctx,
            Err(e) => return Err(e.to_string()),
        };

        let result = context.video();
        let video_sys = match result {
            Ok(video) => video,
            Err(e) => return Err(e),
        };

        Ok(Self {
            sdl_context: context,
            ttf_context: ttf,
            video_subsystem: video_sys,
            system_font_path: font_path,
        })
    }

    pub fn init_graphics(
        &self,
        width: u32,
        height: u32,
        _fullscreen: bool,
    ) -> Result<Graphics, String> {
        // TODO: Fullscreen.
        let result = self
            .video_subsystem
            .window("catio", width, height)
            .position_centered()
            .build();
        let window = match result {
            Ok(wnd) => wnd,
            Err(e) => return Err(e.to_string()),
        };

        let result = window.into_canvas().accelerated().build();
        let canvas = match result {
            Ok(window_canvas) => window_canvas,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Graphics::new(canvas))
    }

    pub fn init_input(&self) -> Result<Input, String> {
        let result = self.sdl_context.event_pump();
        let event_pump = match result {
            Ok(pump) => pump,
            Err(e) => return Err(e),
        };

        Ok(Input::new(event_pump))
    }

    pub fn run<F>(&mut self, frame: F, world: &mut World, input: &mut Input, gfx: &mut Graphics)
    where
        F: Fn(&mut World, &mut Input, &mut Graphics, &Surface, f32) -> bool,
    {
        let path = Path::new(&self.system_font_path);
        let result = self.ttf_context.load_font(path, 16);
        let font = match result {
            Ok(f) => f,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        let mut running = true;
        let mut frame_count = 1u64;
        let mut previous_time = SystemTime::now();
        while running {
            let current_time = SystemTime::now();
            let result = current_time.duration_since(previous_time);
            let dt = match result {
                Ok(dur) => dur,
                Err(e) => {
                    eprintln!("SystemTimeError");
                    e.duration()
                }
            };
            previous_time = current_time;

            let fps_str = format!("{:.4}ms {}", dt.as_secs_f32(), frame_count);
            let fps = font
                .render(&fps_str)
                .blended(Color::RGBA(0, 255, 0, 255))
                .unwrap();

            running = frame(world, input, gfx, &fps, dt.as_secs_f32());
            frame_count += 1;
        }
    }
}
