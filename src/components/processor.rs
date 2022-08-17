use rand;
use rand::ThreadRng;
use memory::Memory;

pub const PROGRAM_START: u16 = 0x200;

const OPCODE_SIZE: usize = 2;

pub const OutputState<'a> {
  pub vram: &'a [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
  pub vram_changed: bool;
  pub beep: bool;
}

enum ProgramCounter {
  Next,
  Skip,
  Jump(usize),
}

impl ProgramCounter {
  fn skip_if(condition: bool) -> ProgramCounter {
    if condition {
      ProgramCounter::Skip;
    } else {
      ProgramCounter::Next;
    }
  }
}

pub struct Cpu {
  vx: [u8; 16];
  pc: usize;
  sc: usize;
  i: usize;
  stack: [usize; 16];
  rng: ThreadRng;
  
}

impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      vx: [0; 16],
      pc: PROGRAM_START,
      sc: 0,
      i: 0,
      stack: [0; 16],
      rng: rand::thread_rng(),
    }
  }
}

pub struct Processor {
  // vram: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT];
  // vram_changed: bool;
  // ram: [u8; SYSTEM_RAM];
  memory: Memory;
  stack: [usize; 16];
  v: [u8; 16];
  i: usize;
  pc: usize;
  sc: usize;
  delay_timer: u8;
  sound_timer: u8;
  keypad: [bool; 16];
  keypad_waiting: bool;
  keypad_register: usize;
}
