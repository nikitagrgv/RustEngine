use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

const NUM_KEYS: usize = Scancode::Num as usize;
const NUM_MOUSE_BUTTONS: usize = 6;

pub struct Input {
    sdl_event_pump: sdl2::EventPump,
    old_keys_states: [bool; NUM_KEYS],
    new_keys_states: [bool; NUM_KEYS],

    old_mouse_buttons_states: [bool; NUM_MOUSE_BUTTONS],
    new_mouse_buttons_states: [bool; NUM_MOUSE_BUTTONS],
}

impl Input {
    pub fn new(sdl_event_pump: sdl2::EventPump) -> Self {
        Self {
            sdl_event_pump,
            old_keys_states: [false; NUM_KEYS],
            new_keys_states: [false; NUM_KEYS],
            old_mouse_buttons_states: [false; NUM_MOUSE_BUTTONS],
            new_mouse_buttons_states: [false; NUM_MOUSE_BUTTONS],
        }
    }

    pub fn update(&mut self) {
        std::mem::swap(&mut self.old_keys_states, &mut self.new_keys_states);
        std::mem::swap(
            &mut self.old_mouse_buttons_states,
            &mut self.new_mouse_buttons_states,
        );

        // keyboard states
        {
            let states = sdl2::keyboard::KeyboardState::new(&self.sdl_event_pump);
            for state in states.scancodes() {
                debug_assert!((state.0 as usize) < self.new_keys_states.len());
                // SAFETY: We have enough length for all scancodes
                unsafe {
                    *self.new_keys_states.get_unchecked_mut(state.0 as usize) = state.1;
                }
            }
        }

        // mouse states
        {
            let states = sdl2::mouse::MouseState::new(&self.sdl_event_pump);
            for state in states.mouse_buttons() {
                debug_assert!((state.0 as usize) < self.new_mouse_buttons_states.len());
                // SAFETY: We have enough length for all buttons
                unsafe {
                    *self
                        .new_mouse_buttons_states
                        .get_unchecked_mut(state.0 as usize) = state.1;
                }
            }
        }
    }

    pub fn is_key_down(&self, scancode: Scancode) -> bool {
        debug_assert!((scancode as usize) < self.new_keys_states.len());
        // SAFETY: We have enough length for all scancodes
        unsafe { *self.new_keys_states.get_unchecked(scancode as usize) }
    }

    pub fn is_key_pressed(&self, scancode: Scancode) -> bool {
        debug_assert!((scancode as usize) < self.new_keys_states.len());
        // SAFETY: We have enough length for all scancodes
        let idx = scancode as usize;
        unsafe {
            *self.new_keys_states.get_unchecked(idx) && !*self.old_keys_states.get_unchecked(idx)
        }
    }

    pub fn is_key_released(&self, scancode: Scancode) -> bool {
        debug_assert!((scancode as usize) < self.new_keys_states.len());
        // SAFETY: We have enough length for all scancodes
        let idx = scancode as usize;
        unsafe {
            !*self.new_keys_states.get_unchecked(idx) && *self.old_keys_states.get_unchecked(idx)
        }
    }

    pub fn is_mouse_down(&self, button: MouseButton) -> bool {
        debug_assert!((button as usize) < self.new_mouse_buttons_states.len());
        // SAFETY: We have enough length for all buttons
        unsafe { *self.new_mouse_buttons_states.get_unchecked(button as usize) }
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        debug_assert!((button as usize) < self.new_mouse_buttons_states.len());
        // SAFETY: We have enough length for all buttons
        let idx = button as usize;
        unsafe {
            *self.new_mouse_buttons_states.get_unchecked(idx)
                && !*self.old_mouse_buttons_states.get_unchecked(idx)
        }
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        debug_assert!((button as usize) < self.new_mouse_buttons_states.len());
        // SAFETY: We have enough length for all buttons
        let idx = button as usize;
        unsafe {
            !*self.new_mouse_buttons_states.get_unchecked(idx)
                && *self.old_mouse_buttons_states.get_unchecked(idx)
        }
    }

    pub fn get_event_pump(&self) -> &sdl2::EventPump {
        &self.sdl_event_pump
    }

    pub fn get_event_pump_mut(&mut self) -> &mut sdl2::EventPump {
        &mut self.sdl_event_pump
    }
}
