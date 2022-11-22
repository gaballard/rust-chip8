extern crate rand;
extern crate sdl2;

mod components;
mod fonts;

use std::env;
use std::fs;

use components::Cpu;

const EMULATOR_NAME: &str = "Rust CHIP-8 Emulator";
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const VIDEO_SCALE: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    let program_data = fs::read(cartridge_filename).unwrap_or(Vec::new());

    let mut cpu = Cpu::new();

    cpu.load_program(program_data);
    cpu.init_video();
    cpu.emulate();
}
