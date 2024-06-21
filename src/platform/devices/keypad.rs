use egui_sdl2_platform::sdl2;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::EventPump;
use sdl2::Sdl;

use crate::EmulatorState;

///
/// Keypad
///
pub struct Keypad {
    event_pump: EventPump,
}

impl Keypad {
    pub fn new(sdl_context: &Sdl) -> Self {
        Keypad {
            event_pump: sdl_context
                .event_pump()
                .expect("SDL2 failed to create event pump in Keypad::new"),
        }
    }

    pub fn read_keypad(&mut self, keys: &mut [bool; 16]) {
        let keypad_state = KeyboardState::new(&mut self.event_pump);

        keys[0x1] = keypad_state.is_scancode_pressed(Scancode::Num1);
        keys[0x2] = keypad_state.is_scancode_pressed(Scancode::Num2);
        keys[0x3] = keypad_state.is_scancode_pressed(Scancode::Num3);
        keys[0xC] = keypad_state.is_scancode_pressed(Scancode::Num4);

        keys[0x4] = keypad_state.is_scancode_pressed(Scancode::Q);
        keys[0x5] = keypad_state.is_scancode_pressed(Scancode::W);
        keys[0x6] = keypad_state.is_scancode_pressed(Scancode::E);
        keys[0xD] = keypad_state.is_scancode_pressed(Scancode::R);

        keys[0x7] = keypad_state.is_scancode_pressed(Scancode::A);
        keys[0x8] = keypad_state.is_scancode_pressed(Scancode::S);
        keys[0x9] = keypad_state.is_scancode_pressed(Scancode::D);
        keys[0xE] = keypad_state.is_scancode_pressed(Scancode::F);

        keys[0xA] = keypad_state.is_scancode_pressed(Scancode::Z);
        keys[0x0] = keypad_state.is_scancode_pressed(Scancode::X);
        keys[0xB] = keypad_state.is_scancode_pressed(Scancode::C);
        keys[0xF] = keypad_state.is_scancode_pressed(Scancode::V);
    }

    pub fn read_host_keypad(&mut self) -> EmulatorState {
        for event in self.event_pump.poll_iter() {
            match event {
                // Quit
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return EmulatorState::Quit,

                // Reset
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Delete),
                    ..
                } => return EmulatorState::Reset,

                // Toggle debug mode
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => return EmulatorState::DebugMode,

                // Step through next instruction (debug mode only)
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    repeat: false,
                    ..
                } => return EmulatorState::Step,

                // Continue
                _ => return EmulatorState::Running,
            }
        }
        EmulatorState::Running
    }
}
