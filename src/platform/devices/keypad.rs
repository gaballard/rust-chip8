extern crate sdl2;

use sdl2::EventPump;
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::{KeyboardState, Keycode, Scancode};

use crate::Actions;
use crate::chip8::ScrollDir;
use crate::chippy::Settings;

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

    pub fn read_host_keypad(&mut self, settings: &Settings) -> Actions {
        for event in self.event_pump.poll_iter() {
            match event {
                // Quit
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Actions::Quit,

                // Reset
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Delete),
                    ..
                } => return Actions::Reset,

                // Toggle debug mode
                Event::KeyDown {
                    keycode: Some(Keycode::T),
                    ..
                } => return Actions::ToggleDebugMode,

                Event::KeyDown {
                    keycode: Some(Keycode::Backslash),
                    ..
                } => return Actions::ToggleDebugPanel,

                // Step through next instruction (debug mode only)
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    repeat: false,
                    ..
                } => return Actions::Step,

                // Video scaling
                Event::KeyDown {
                    keycode: Some(Keycode::LeftBracket),
                    ..
                } => return Actions::DecVideoScale,

                Event::KeyDown {
                    keycode: Some(Keycode::RightBracket),
                    ..
                } => return Actions::IncVideoScale,

                // Set hires mode
                Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
                } => return Actions::ToggleHiresMode,

                // Scroll display
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => return Actions::ScrollScreen(ScrollDir::Down, 1),

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => return Actions::ScrollScreen(ScrollDir::Up, 1),

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => return Actions::ScrollScreen(ScrollDir::Right, 1),

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => return Actions::ScrollScreen(ScrollDir::Left, 1),

                // Toggle CRT fade effect
                Event::KeyDown {
                    keycode: Some(Keycode::J),
                    ..
                } => return Actions::ToggleCrtFade,

                Event::KeyDown {
                    keycode: Some(Keycode::Num7),
                    ..
                } => {
                    return Actions::SetCrtFadeSpeed(settings.crt_fade_rate_slow);
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Num8),
                    ..
                } => return Actions::SetCrtFadeSpeed(settings.crt_fade_rate_medium),

                Event::KeyDown {
                    keycode: Some(Keycode::Num9),
                    ..
                } => return Actions::SetCrtFadeSpeed(settings.crt_fade_rate_fast),

                // Switch color palette
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => return Actions::CycleColorPalette,

                // Continue
                _ => return Actions::Running,
            }
        }
        Actions::Running
    }
}
