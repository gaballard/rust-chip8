extern crate rand;
extern crate sdl2;

mod components;
mod fonts;

use std::thread;
use std::time::Duration;
use std::env;

use components::Processor;
use components::Memory;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SYSTEM_RAM: usize = 4092;

fn main() {
    let sleep_duration = Duration::from_millis(2);

    let sdl_context = sdl2::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    let mut Process::new();

    
}
