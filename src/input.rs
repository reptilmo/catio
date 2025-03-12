use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Input {
    event_pump: EventPump,
    keys_pressed: HashSet<Scancode>,
    keys_were_pressed: HashSet<Scancode>,
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_were_pressed: HashSet<MouseButton>,
    mouse_pos_x: i32,
    mouse_pos_y: i32,
}

impl Input {
    pub fn new(pump: EventPump) -> Self {
        Self {
            event_pump: pump,
            keys_pressed: HashSet::<Scancode>::default(),
            keys_were_pressed: HashSet::<Scancode>::default(),
            mouse_buttons_pressed: HashSet::<MouseButton>::default(),
            mouse_buttons_were_pressed: HashSet::<MouseButton>::default(),
            mouse_pos_x: 0,
            mouse_pos_y: 0,
        }
    }

    pub fn update(&mut self) {
        self.event_pump.pump_events();
        self.keys_were_pressed = self.keys_pressed.clone();
        self.keys_pressed = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .collect();

        self.mouse_buttons_were_pressed = self.mouse_buttons_pressed.clone();
        self.mouse_buttons_pressed = self
            .event_pump
            .mouse_state()
            .pressed_mouse_buttons()
            .collect();

        self.mouse_pos_x = self.event_pump.mouse_state().x();
        self.mouse_pos_y = self.event_pump.mouse_state().y();
    }

    pub fn key_pressed(&self, scancode: Scancode) -> bool {
        self.keys_pressed.contains(&scancode)
    }

    pub fn key_was_pressed(&self, scancode: Scancode) -> bool {
        self.keys_were_pressed.contains(&scancode)
    }

    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn mouse_was_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_were_pressed.contains(&button)
    }

    pub fn mouse_position(&self) -> (i32, i32) {
        (self.mouse_pos_x, self.mouse_pos_y)
    }
}
