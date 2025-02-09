extern crate sdl2;

use sdl2::{Sdl, VideoSubsystem};
use std::time::SystemTime;

use crate::graphics::Graphics;
use crate::input::Input;

pub struct System {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    running: bool,
}

impl System {
    pub fn init() -> Result<Self, String> {
        let result = sdl2::init();
        let context = match result {
            Ok(sdl) => sdl,
            Err(e) => return Err(e),
        };

        let result = context.video();
        let video_sys = match result {
            Ok(video) => video,
            Err(e) => return Err(e),
        };

        Ok(Self {
            sdl_context: context,
            video_subsystem: video_sys,
            running: true,
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
            .window("", width, height)
            .position_centered()
            .build();
        let window = match result {
            Ok(wnd) => wnd,
            Err(e) => return Err(e.to_string()),
        };

        let result = window.into_canvas().build();
        let canvas = match result {
            Ok(window_canvas) => window_canvas,
            Err(e) => return Err(e.to_string()),
        };

        Ok(Graphics::new(canvas))
    }

    pub fn init_input(&mut self) -> Result<Input, String> {
        let result = self.sdl_context.event_pump();
        let event_pump = match result {
            Ok(pump) => pump,
            Err(e) => return Err(e),
        };

        Ok(Input::new(event_pump))
    }

    pub fn run<F>(&mut self, frame: F, input: &mut Input, gfx: &mut Graphics)
    where
        F: Fn(&mut Input, &mut Graphics, f32) -> bool,
    {
        let mut previous_time = SystemTime::now();
        while self.running {
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
            self.running = frame(input, gfx, dt.as_secs_f32());
        }
    }
}
