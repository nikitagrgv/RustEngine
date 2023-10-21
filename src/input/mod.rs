use sdl2::keyboard::Scancode;

pub struct Input {
    sdl_event_pump: sdl2::EventPump,
}

impl Input {
    pub fn new(sdl_event_pump: sdl2::EventPump) -> Self {
        Self { sdl_event_pump }
    }

    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        let states = sdl2::keyboard::KeyboardState::new(&self.sdl_event_pump);
        states.is_scancode_pressed(scancode)
    }

    pub fn get_event_pump(&self) -> &sdl2::EventPump {
        &self.sdl_event_pump
    }

    pub fn get_event_pump_mut(&mut self) -> &mut sdl2::EventPump {
        &mut self.sdl_event_pump
    }
}
