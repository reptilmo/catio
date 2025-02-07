use sdl2::keyboard::Scancode;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Input {
    keys_pressed: HashSet<Scancode>,
    keys_were_pressed: HashSet<Scancode>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::<Scancode>::default(),
            keys_were_pressed: HashSet::<Scancode>::default(),
        }
    }

    pub fn update(&mut self, event_pump: &EventPump) {
        self.keys_were_pressed = self.keys_pressed.clone();
        self.keys_pressed = event_pump.keyboard_state().pressed_scancodes().collect();
    }

    pub fn key_pressed(&self, scancode: Scancode) -> bool {
        self.keys_pressed.contains(&scancode)
    }

    pub fn key_was_pressed(&self, scancode: Scancode) -> bool {
        self.keys_were_pressed.contains(&scancode)
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}
