extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};
use sdl2::EventPump;
use sdl2::Sdl;

use crate::{models::InputBuffer, EmulatorState};

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

    pub fn read_keypad(&mut self, keys: &mut InputBuffer) {
        let keypad_state = KeyboardState::new(&mut self.event_pump);

        keys.set(0x0, keypad_state.is_scancode_pressed(Scancode::Num1));
        keys.set(0x1, keypad_state.is_scancode_pressed(Scancode::Num2));
        keys.set(0x2, keypad_state.is_scancode_pressed(Scancode::Num3));
        keys.set(0x3, keypad_state.is_scancode_pressed(Scancode::Num4));

        keys.set(0x4, keypad_state.is_scancode_pressed(Scancode::Q));
        keys.set(0x5, keypad_state.is_scancode_pressed(Scancode::W));
        keys.set(0x6, keypad_state.is_scancode_pressed(Scancode::E));
        keys.set(0x7, keypad_state.is_scancode_pressed(Scancode::R));

        keys.set(0x8, keypad_state.is_scancode_pressed(Scancode::A));
        keys.set(0x9, keypad_state.is_scancode_pressed(Scancode::S));
        keys.set(0xA, keypad_state.is_scancode_pressed(Scancode::D));
        keys.set(0xB, keypad_state.is_scancode_pressed(Scancode::F));

        keys.set(0xC, keypad_state.is_scancode_pressed(Scancode::Z));
        keys.set(0xD, keypad_state.is_scancode_pressed(Scancode::X));
        keys.set(0xE, keypad_state.is_scancode_pressed(Scancode::C));
        keys.set(0xF, keypad_state.is_scancode_pressed(Scancode::V));
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
