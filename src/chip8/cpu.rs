mod input_buffer;
mod program_counter;
mod registers;
mod stack;

use input_buffer::InputBuffer;
use log::debug;
use program_counter::ProgramCounter;
use rand::{Rng, rngs::ThreadRng};
use registers::Registers;
use stack::Stack;

use super::{Memory, VideoMemory};

use crate::{
    chip8::{ScrollDir, sprite::SpriteKey},
    chippy::InstructionSet,
    constants,
    fonts::CHIP8_FONTS,
};

pub type Opcode = (u8, u8, u8, u8);

///
/// CPU
///
#[must_use]
pub struct Cpu {
    pub instruction_set: InstructionSet,
    pub quit_flag: bool,
    pub cycle: usize,
    pub v: Registers,
    pub i: u16,
    pc: ProgramCounter,
    stack: Stack,
    delay_timer: u8,
    pub sound_timer: u8,
    rng: ThreadRng,
    pub keys: InputBuffer,
    key_register: u8,
    waiting_for_key: bool,
    key_down: u8,
    keys_down: [bool; 16],
    keys_up: [bool; 16],
    pub ram: Memory,
    pub vram: VideoMemory,
}

impl Cpu {
    pub fn new(instruction_set: InstructionSet) -> Self {
        let mut cpu = Self {
            instruction_set,
            quit_flag: false,
            cycle: 0,
            v: Registers::new(),
            i: 0,
            pc: ProgramCounter::new(),
            stack: Stack::new(),
            rng: rand::thread_rng(),
            delay_timer: 0,
            sound_timer: 0,
            keys: InputBuffer::new(),
            key_register: 0,
            waiting_for_key: false,
            key_down: 0,
            keys_down: [false; 16],
            keys_up: [true; 16],
            ram: Memory::new(),
            vram: VideoMemory::new(),
        };

        let mut i = constants::FONT_START_ADDR;
        for font_char in &CHIP8_FONTS {
            cpu.ram.write(i, *font_char);
            i += 1;
        }

        cpu
    }

    pub fn reset(&mut self) {
        self.ram.clear();
        self.vram.clear();
        self.v.clear();
        self.i = 0;
        self.pc.reset();
        self.stack.clear();
        self.rng = rand::thread_rng();
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys.clear();
        self.keys_down = [false; 16];
        self.keys_up = [true; 16];
        self.waiting_for_key = false;
    }

    #[inline]
    pub fn get_pc(&self) -> &ProgramCounter {
        &self.pc
    }

    #[inline]
    pub fn load_program(&mut self, data: &Vec<u8>) {
        let mut address = constants::PROGRAM_START_ADDR;
        for v in data {
            self.ram.write(address, *v);
            address += 1;
        }
    }

    fn dec_timers(&mut self) {
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }

    pub fn tick(&mut self) {
        self.cycle = self.cycle.wrapping_add(1);

        self.dec_timers();

        if self.wait_for_keys() {
            return;
        }

        let opcode = self.read_instruction();
        self.pc.next();
        self.execute_instruction(opcode);
    }

    fn wait_for_keys(&mut self) -> bool {
        if self.waiting_for_key {
            // All execution stops until a key is pressed and then released, then the value of that key is stored in Vx.
            // However, timers still count down
            if self.key_down > 0 {
                if !*self.keys.get_key(self.key_down as usize) {
                    self.waiting_for_key = false;
                    self.v.write(self.key_register, self.key_down as u8);
                    self.key_register = 0;
                    return false;
                } else {
                    return true;
                }
            } else {
                for key in 0..15 as u8 {
                    if *self.keys.get_key(key as usize) {
                        // key down
                        self.key_down = key;
                        break;
                    }
                }
                return true;
            }
        } else {
            return false;
        }
    }

    #[inline]
    fn read_instruction(&mut self) -> u16 {
        (*self.ram.read(self.pc.address) as u16) << 8 | (*self.ram.read(self.pc.address + 1) as u16)
    }

    fn execute_instruction(&mut self, instruction: u16) {
        let opcode: Opcode = (
            ((instruction & 0xF000) >> 12) as u8,
            ((instruction & 0x0F00) >> 8) as u8,
            ((instruction & 0x00F0) >> 4) as u8,
            (instruction & 0x000F) as u8,
        );

        let x = opcode.1.try_into().unwrap();
        let y = opcode.2.try_into().unwrap();

        let n: u8 = opcode.3.try_into().unwrap();
        let kk = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        match opcode {
            (0x0, 0x0, 0xC, 0x0) if self.instruction_set.is_hp48() => self.no_op(opcode), // SCHIP only
            (0x0, 0x0, 0xC, _) => self.scd(n), // SCHIP only
            (0x0, 0x0, 0xD, _) => self.scu(n), // SCHIP only
            (0x0, 0x0, 0xE, 0x0) => self.cls(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x0, 0x0, 0xF, 0xB) => self.scr(),  // SCHIP only
            (0x0, 0x0, 0xF, 0xC) => self.scl(),  // SCHIP only
            (0x0, 0x0, 0xF, 0xD) => self.exit(), // SCHIP only
            (0x0, 0x0, 0xF, 0xE) => self.low(),  // SCHIP only
            (0x0, 0x0, 0xF, 0xF) => self.high(), // SCHIP only
            (0x1, _, _, _) => self.jp(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.se_vx(x, kk),
            (0x4, _, _, _) => self.sne_vx(x, kk),
            (0x5, _, _, 0x0) => self.se_vx_vy(x, y),
            (0x6, _, _, _) => self.ld_vx(x, kk),
            (0x7, _, _, _) => self.add_vx(x, kk),
            (0x8, _, _, 0x0) => self.ld_vx_vy(x, y),
            (0x8, _, _, 0x1) => self.or_vx_vy(x, y), // SCHIP variation
            (0x8, _, _, 0x2) => self.and_vx_vy(x, y), // SCHIP variation
            (0x8, _, _, 0x3) => self.xor_vx_vy(x, y), // SCHIP variation
            (0x8, _, _, 0x4) => self.add_vx_vy(x, y),
            (0x8, _, _, 0x5) => self.sub_vx_vy(x, y),
            (0x8, _, _, 0x6) => self.shr_vx_vy(x, y), // SCHIP variation
            (0x8, _, _, 0x7) => self.subn_vx_vy(x, y),
            (0x8, _, _, 0xE) => self.shl_vx_vy(x, y), // SCHIP variation
            (0x9, _, _, 0x0) => self.sne_vx_vy(x, y),
            (0xA, _, _, _) => self.ld_i(nnn),
            (0xB, _, _, _) if !self.instruction_set.is_schip() => self.jp_v0(nnn), // CHIP-8 only
            (0xB, _, _, _) if self.instruction_set.is_schip() => self.jp_vx(x, nnn), // SCHIP only
            (0xC, _, _, _) => self.rnd_vx(x, kk),
            (0xD, _, _, 0x0) => self.drw_vx_vy_0(x, y), // SCHIP only
            (0xD, _, _, _) => self.drw_vx_vy_n(x, y, n), // SCHIP variation
            (0xE, _, 0x9, 0xE) => self.skp_vx(x),
            (0xE, _, 0xA, 0x1) => self.sknp_vx(x),
            (0xF, _, 0x0, 0x7) => self.ld_vx_dt(x),
            (0xF, _, 0x0, 0xA) => self.ld_vx_k(x),
            (0xF, _, 0x1, 0x5) => self.ld_dt_vx(x),
            (0xF, _, 0x1, 0x8) => self.ld_st_vx(x),
            (0xF, _, 0x1, 0xE) => self.add_i_vx(x),
            (0xF, _, 0x2, 0x9) => self.ld_f_vx(x),
            (0xF, _, 0x3, 0x0) => self.ld_hf_vx(x), // SCHIP only
            (0xF, _, 0x3, 0x3) => self.ld_b_vx(x),
            (0xF, _, 0x5, 0x5) => self.ld_i_vx(x), // SCHIP variation
            (0xF, _, 0x6, 0x5) => self.ld_vx_i(x), // SCHIP variation
            (0xF, _, 0x7, 0x5) => self.ld_r_vx(x), // SCHIP only
            (0xF, _, 0x8, 0x5) => self.ld_vx_r(x), // SCHIP only
            _ => self.no_op(opcode),
        }
    }

    ///
    /// 00Cn - SCD (SCHIP-1.1 only)
    ///
    /// Scroll display N lines down.
    ///
    pub fn scd(&mut self, n: u8) {
        debug!("00Cn - SCD {}", n);
        if self.instruction_set != InstructionSet::Schip1point1 {
            debug!("Only available in SCHIP 1.1 mode");
            return;
        } else if !self.vram.hires_mode {
            debug!("Only available in hires mode");
            return;
        }
        self.vram.scroll(ScrollDir::Down, n as usize);
    }

    ///
    /// 00Dn - SCU (SCHIP-1.1 only)
    ///
    /// Scroll display N lines up.
    ///
    pub fn scu(&mut self, n: u8) {
        debug!("00Dn - SCU {}", n);
        if self.instruction_set != InstructionSet::Schip1point1 {
            debug!("Only available in SCHIP 1.1 mode");
            return;
        } else if !self.vram.hires_mode {
            debug!("Only available in hires mode");
            return;
        }
        self.vram.scroll(ScrollDir::Up, n as usize);
    }

    ///
    /// 00E0 - CLS
    ///
    /// Clear the display.
    ///
    pub fn cls(&mut self) {
        debug!("00E0 - CLS");
        self.vram.clear();
    }

    ///
    /// 00EE - RET
    ///
    /// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
    ///
    pub fn ret(&mut self) {
        debug!("00EE - RET");
        self.pc.jump(self.stack.pop());
    }

    ///
    /// 00FB - SCR (SCHIP-1.1 only)
    ///
    /// Scroll display 4 pixels to the right.
    ///
    pub fn scr(&mut self) {
        debug!("00FB - SCR");
        if self.instruction_set != InstructionSet::Schip1point1 {
            debug!("Only available in SCHIP 1.1 mode");
            return;
        } else if !self.vram.hires_mode {
            debug!("Only available in hires mode");
            return;
        }
        self.vram.scroll(ScrollDir::Right, 4);
    }

    ///
    /// 00FC - SCL (SCHIP-1.1 only)
    ///
    /// Scroll display 4 pixels to the left.
    ///
    pub fn scl(&mut self) {
        debug!("00FC - SCL");
        if self.instruction_set != InstructionSet::Schip1point1 {
            debug!("Only available in SCHIP 1.1 mode");
            return;
        } else if !self.vram.hires_mode {
            debug!("Only available in hires mode");
            return;
        }
        self.vram.scroll(ScrollDir::Left, 4);
    }

    ///
    /// 00FD - EXIT (SCHIP only)
    ///
    /// Exit the interpreter.
    ///
    pub fn exit(&mut self) {
        debug!("00FD - EXIT");
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }
        self.quit_flag = true;
    }

    ///
    /// 00FE - LOW (SCHIP only)
    ///
    /// Disable hires screen mode.
    ///
    /// The original SCHIP-1.x did not clear the screen, leading to artifacts.
    ///
    pub fn low(&mut self) {
        debug!("00FE - LOW");
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }
        self.vram.hires_mode = false;
    }

    ///
    /// 00FF - HIGH (SCHIP only)
    ///
    /// Enable hires screen mode.
    ///
    /// The original SCHIP-1.x did not clear the screen, leading to artifacts.
    ///
    pub fn high(&mut self) {
        debug!("00FF - HIGH");
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }
        self.vram.hires_mode = true;
    }

    ///
    /// 1nnn - JP
    ///
    /// The interpreter sets the program counter to nnn.
    ///
    pub fn jp(&mut self, address: u16) {
        debug!("1nnn - JP address: {}", address);
        self.pc.jump(address);
    }

    ///
    /// 2nnn - CALL
    ///
    /// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    ///
    pub fn call(&mut self, address: u16) {
        debug!("2nnn - CALL address: {}", address);
        // self.pc.next();
        self.stack.push(self.pc.address);
        self.pc.jump(address);
    }

    ///
    /// 3xkk - SE Vx
    ///
    /// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
    ///
    pub fn se_vx(&mut self, vx: u8, kk: u8) {
        debug!("3xkk - SE V{} byte: {}", vx, (kk));
        if self.v.read(vx) == &kk {
            self.pc.next();
        }
    }

    ///
    /// 4xkk - SNE Vx
    ///
    /// // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
    ///
    pub fn sne_vx(&mut self, vx: u8, kk: u8) {
        debug!("4xkk - SNE V{} byte: {}", vx, (kk));
        if self.v.read(vx) != &kk {
            self.pc.next();
        }
    }

    ///
    /// 5xy0 - SE Vx, Vy
    ///
    /// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
    ///
    pub fn se_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("5xy0 - SE V{}, V{}", vx, vy);
        if self.v.read(vx) == self.v.read(vy) {
            self.pc.next();
        }
    }

    ///
    /// 6xkk - LD Vx
    ///
    /// The interpreter puts the value kk into register Vx.
    ///
    pub fn ld_vx(&mut self, vx: u8, kk: u8) {
        debug!("6xkk - LD V{} byte: {}", vx, (kk));
        self.v.write(vx, kk);
    }

    ///
    /// 7xkk - ADD Vx
    ///
    /// Adds the value kk to the value of register Vx, then stores the result in Vx.
    ///
    pub fn add_vx(&mut self, vx: u8, kk: u8) {
        debug!("7xkk - ADD V{} byte: {}", vx, (kk));
        // TODO: does this wrap?
        self.v.write(vx, self.v.read(vx).wrapping_add(kk));
    }

    ///
    /// 8xy0 - LD Vx, Vy
    ///
    /// Stores the value of register Vy in register Vx.
    ///
    pub fn ld_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy0 - LD V{}, V{}", vx, vy);
        self.v.write(vx, *self.v.read(vy));
    }

    ///
    /// 8xy1 - OR Vx, Vy
    ///
    /// Set Vx = Vx OR Vy.
    ///
    pub fn or_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy1 - OR V{}, V{}", vx, vy);
        self.v.write(vx, self.v.read(vx) | self.v.read(vy));
        if self.instruction_set.is_cosmac() {
            // COSMAC based variants will reset VF
            self.v.write(0xF, 0);
        }
    }

    ///
    /// 8xy2 - AND Vx, Vy
    ///
    /// Set Vx = Vx AND Vy.
    ///
    pub fn and_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy2 - AND V{}, V{}", vx, vy);
        self.v.write(vx, self.v.read(vx) & self.v.read(vy));
        if self.instruction_set.is_cosmac() {
            // COSMAC based variants will reset VF
            self.v.write(0xF, 0);
        }
    }

    ///
    /// 8xy3 - XOR Vx, Vy
    ///
    /// Set Vx = Vx XOR Vy.
    ///
    pub fn xor_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy3 - XOR V{}, V{}", vx, vy);
        if self.instruction_set == InstructionSet::Chip8D6800 {
            debug!(
                "Command not supported by {}",
                self.instruction_set.get_title()
            );
            return;
        }

        self.v.write(vx, self.v.read(vx) ^ self.v.read(vy));
        if self.instruction_set.is_cosmac() {
            // COSMAC based variants will reset VF
            self.v.write(0xF, 0);
        }
    }

    ///
    /// 8xy4 - ADD Vx, Vy
    ///
    /// Add Vy to Vx, VF is set to 1 if an overflow happened, to 0 if not, even if X=F!
    ///
    /// Set Vx = Vx + Vy, set VF = carry
    ///
    pub fn add_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy4 - ADD V{}, V{}", vx, vy);
        let (res, overflow) = self.v.read(vx).overflowing_add(*self.v.read(vy));
        self.v.write(vx, res);
        self.v.write(0xF, if overflow { 1 } else { 0 });
    }

    ///
    /// 8xy5 - SUB Vx, Vy
    ///
    /// Subtract Vy from Vx, VF is set to 0 if an underflow happened, to 1 if not, even if X=F!
    ///
    pub fn sub_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy5 - SUB V{}, V{}", vx, vy);
        let (res, underflow) = self.v.read(vx).overflowing_sub(*self.v.read(vy));
        self.v.write(vx, res);
        self.v.write(0xF, if underflow { 0 } else { 1 });
    }

    ///
    /// 8xy6 - SHR Vx, Vy (SCHIP Variation)
    ///
    /// CHIP-8
    ///     Set Vx to Vy and shift Vx one bit to the right, set VF to the bit shifted out, even if X=F!
    /// CHIP-48, SCHIP 1.x
    ///     Same as CHIP-8, but don't set Vx to Vy
    ///
    pub fn shr_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy6 - SHR V{}, V{}", vx, vy);
        if self.instruction_set == InstructionSet::Chip8D6800 {
            debug!(
                "Command not supported by {}",
                self.instruction_set.get_title()
            );
            return;
        }

        let lsb = self.v.read(vx) & 0b0000000000000001;
        if !self.instruction_set.is_schip() && self.instruction_set != InstructionSet::Chip48 {
            self.v.write(vx, *self.v.read(vy));
        }
        self.v.write(vx, *self.v.read(vx) >> 1);
        self.v.write(0xF, lsb);
    }

    ///
    /// 8xy7 - SUBN Vx, Vy
    ///
    /// Set Vx to the result of subtracting Vx from Vy, VF is set to 0 if an underflow happened, to 1 if not, even if X=F!
    ///
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
    ///
    pub fn subn_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xy7 - SUBN V{}, V{}", vx, vy);
        if self.instruction_set == InstructionSet::Chip8D6800 {
            debug!("Not supported by {}", self.instruction_set.get_title());
            return;
        }

        let (res, underflow) = self.v.read(vy).overflowing_sub(*self.v.read(vx));
        self.v.write(vx, res);
        self.v.write(0xF, if underflow { 0 } else { 1 });
    }

    ///
    /// 8xyE - SHL Vx, Vy (SCHIP Variation)
    ///
    /// CHIP-8
    ///     Set Vx to Vy and shift Vx one bit to the left, set VF to the bit shifted out, even if X=F!
    /// CHIP-48, SCHIP 1.x
    ///     If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
    ///
    pub fn shl_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("8xyE - SHL V{}, V{}", vx, vy);
        if self.instruction_set == InstructionSet::Chip8D6800 {
            debug!("Not supported by {}", self.instruction_set.get_title());
            return;
        }

        let msb = self.v.read(vx) >> 7 & 1;
        if !self.instruction_set.is_hp48() {
            self.v.write(vx, *self.v.read(vy));
        }
        self.v.write(vx, *self.v.read(vx) << 1);
        self.v.write(0xF, msb);
    }

    ///
    /// 9xy0 - SNE Vx, Vy
    ///
    /// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
    ///
    pub fn sne_vx_vy(&mut self, vx: u8, vy: u8) {
        debug!("9xy0 - SNE V{}, V{}", vx, vy);
        if self.v.read(vx) != self.v.read(vy) {
            // self.next_instruction();
            self.pc.next();
        }
    }

    ///
    /// Annn - LD I
    ///
    /// I is set to nnn.
    ///
    pub fn ld_i(&mut self, address: u16) {
        debug!("Annn - LD I address: {}", address);
        self.i = address;
    }

    ///
    /// Bnnn - JP V0
    ///
    /// The program counter is set to nnn plus the value of V0.
    ///
    pub fn jp_v0(&mut self, address: u16) {
        debug!("Bnnn - JP V0 address: {}", address);
        if self.instruction_set.is_schip() {
            debug!("Not available in SCHIP mode");
            return;
        }
        self.pc.jump(*self.v.read(0) as u16 + address);
    }

    ///
    /// Bxnn - JP VX (SCHIP Only)
    ///
    /// The program counter is set to xnn plus the value of VX.
    ///
    pub fn jp_vx(&mut self, vx: u8, address: u16) {
        debug!("Bxnn - JP VX address: {}", address);
        if self.instruction_set != InstructionSet::Schip1point0
            && self.instruction_set != InstructionSet::Schip1point1
        {
            debug!("Only available in SCHIP-1.x mode");
            return;
        }
        self.pc.jump(*self.v.read(vx) as u16 + address);
    }

    ///
    /// Cxkk - RND Vx
    ///
    /// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
    ///
    pub fn rnd_vx(&mut self, vx: u8, value: u8) {
        debug!("Cxkk - RND V{}, byte: {}", vx, value);
        let rnum: u8 = self.rng.r#gen();
        self.v.write(vx, rnum & value);
    }

    ///
    /// Dxyn - DRW Vx, Vy, N (SCHIP Variation)
    ///
    /// Display 8xN pixel sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    pub fn drw_vx_vy_n(&mut self, vx: u8, vy: u8, sprite_len: u8) {
        debug!(
            "D{}{}{} - DRW V{}, V{}, {}",
            vx, vy, sprite_len, vx, vy, sprite_len
        );

        let sprite_key: SpriteKey = SpriteKey(vx, vy, self.i, self.i + sprite_len as u16);
        let sprite_data = self.ram.read_slice(self.i, sprite_len as u16);

        if !self.vram.has_sprite(&sprite_key) {
            self.vram
                .add_sprite(&sprite_key, 8, sprite_len, sprite_data);
        }

        let x = *self.v.read(vx) as usize % self.vram.get_screen_width();
        let y = *self.v.read(vy) as usize % self.vram.get_screen_height();

        self.vram.place_sprite(&sprite_key, x, y);
        let collisions = self.vram.draw_sprite(
            &sprite_key,
            if self.instruction_set.is_schip() {
                false
            } else {
                true
            },
        );

        self.v.write(0xF, 0);

        if collisions > 0 {
            self.v.write(
                0xF,
                if self.instruction_set.is_schip() {
                    collisions
                } else {
                    self.v.read(0xF) | 1
                },
            );
            self.vram.clear_collisions();
        }
    }

    ///
    /// Dxy0 - DRW Vx, Vy, 0 (SCHIP only)
    ///
    /// Display 16x16 pixel sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    pub fn drw_vx_vy_0(&mut self, vx: u8, vy: u8) {
        debug!("D{}{}0 - DRW V{}, V{}, 0", vx, vy, vx, vy);

        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }

        let sprite_width: u8 = 16;
        let sprite_height: u8 = 16;

        let sprite_key: SpriteKey = SpriteKey(vx, vy, self.i, self.i + sprite_height as u16);
        let sprite_data = self.ram.read_slice(self.i, sprite_height as u16);

        if !self.vram.has_sprite(&sprite_key) {
            self.vram
                .add_sprite(&sprite_key, sprite_width, sprite_height, sprite_data);
        }

        let x = *self.v.read(vx) as usize % self.vram.get_screen_width();
        let y = *self.v.read(vy) as usize % self.vram.get_screen_height();

        self.vram.place_sprite(&sprite_key, x, y);
        let collisions = self.vram.draw_sprite(
            &sprite_key,
            if self.instruction_set.is_schip() {
                false
            } else {
                true
            },
        );

        self.v.write(0xF, 0);

        if collisions > 0 {
            self.v.write(
                0xF,
                if self.instruction_set.is_schip() {
                    collisions
                } else {
                    self.v.read(0xF) | 1
                },
            );
            self.vram.clear_collisions();
        }
    }

    ///
    /// Ex9E - SKP Vx
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
    ///
    pub fn skp_vx(&mut self, vx: u8) {
        debug!("Ex9E - SKP V{}", vx);
        if *self.keys.get_key(*self.v.read(vx) as usize) {
            self.pc.next();
        }
    }

    ///
    /// ExA1 - SKNP Vx
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
    ///
    pub fn sknp_vx(&mut self, vx: u8) {
        debug!("ExA1 - SKNP V{}", vx);
        if !self.keys.get_key(*self.v.read(vx) as usize) {
            self.pc.next();
        }
    }

    ///
    /// Fx07 - LD Vx, DT
    ///
    /// The delay timer value is placed into Vx.
    ///
    pub fn ld_vx_dt(&mut self, vx: u8) {
        debug!("Fx07 - LD V{}, DT", vx);
        self.v.write(vx, self.delay_timer);
    }

    ///
    /// Fx0A - LD Vx, K
    ///
    /// All execution stops until a key is pressed and then released, then the value of that key is stored in Vx.
    ///
    pub fn ld_vx_k(&mut self, vx: u8) {
        debug!("Fx0A - LD V{}, K", vx);
        if !self.waiting_for_key {
            self.key_register = vx;
            self.waiting_for_key = true;
        }
    }

    ///
    /// Fx15 - LD DT Vx
    ///
    /// Delay timer is set equal to the value of Vx.
    ///
    pub fn ld_dt_vx(&mut self, vx: u8) {
        debug!("Fx15 - LD DT V{}", vx);
        self.delay_timer = *self.v.read(vx);
    }

    ///
    /// Fx18 - LD ST Vx
    ///
    /// Sound timer is set equal to the value of Vx.
    ///
    pub fn ld_st_vx(&mut self, vx: u8) {
        debug!("Fx18 - LD ST V{}", vx);
        self.sound_timer = *self.v.read(vx);
    }

    ///
    /// Fx1E - ADD I Vx
    ///
    /// The values of I and Vx are added, and the results are stored in I.
    ///
    pub fn add_i_vx(&mut self, vx: u8) {
        debug!("Fx1E - ADD I V{}", vx);
        self.i = self.i.wrapping_add(*self.v.read(vx) as u16);
    }

    ///
    /// Fx29 - LD F, Vx
    ///
    /// The value of I is set to the 5-byte sprite corresponding to the hex character in Vx.
    ///
    pub fn ld_f_vx(&mut self, vx: u8) {
        debug!("Fx29 - LD F V{}", vx);
        self.i = constants::FONT_START_ADDR.wrapping_add((*self.v.read(vx) * 5) as u16);
    }

    ///
    /// Fx30 - LD HF, Vx (SCHIP Only)
    ///
    /// The value of I is set to the 10-byte sprite corresponding to the decimal value of Vx (0-9).
    ///
    pub fn ld_hf_vx(&mut self, vx: u8) {
        debug!("Fx30 - LD HF, V{}", vx);
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        } else if vx > 0x9 {
            debug!("Value in V{} is not a decimal character!", vx);
            return;
        }
        self.i = constants::LARGE_FONT_START_ADDR.wrapping_add((*self.v.read(vx) * 10) as u16);
    }

    ///
    /// Fx33 - LD B Vx
    ///
    /// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
    ///
    pub fn ld_b_vx(&mut self, vx: u8) {
        debug!("Fx33 - LD B V{}", vx);
        let i = self.i;
        let val = *self.v.read(vx);

        self.ram.write(i, val / 100);
        self.ram.write(i + 1, (val % 100) / 10);
        self.ram.write(i + 2, val % 10);
    }

    ///
    /// Fx55 - LD I Vx (SCHIP Variation)
    ///
    /// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
    ///  
    /// CHIP-8:
    ///     I is then incremented by x + 1.
    /// CHIP-48, SCHIP 1.0:
    ///     I is then incremented by x.
    /// SCHIP-1.1, SCHIP-MODERN
    ///     I is not incremented.
    ///
    pub fn ld_i_vx(&mut self, vx: u8) {
        debug!("Fx55 - LD I V{}", vx);
        for i in 0..(vx + 1) as u16 {
            self.ram.write(self.i + i, *self.v.read(i as u8));
        }

        if self.instruction_set == InstructionSet::Chip48
            || self.instruction_set == InstructionSet::Schip1point0
        {
            self.i = self.i + vx as u16;
        } else if self.instruction_set != InstructionSet::Schip1point1
            && self.instruction_set != InstructionSet::SchipModern
        {
            self.i = self.i + vx as u16 + 1;
        }
    }

    ///
    /// Fx65 - LD Vx, I (SCHIP Variation)
    ///
    /// The interpreter reads values from memory starting at location I into registers V0 through Vx. I is then incremented.
    ///  
    /// CHIP-8:
    ///     I is then incremented by x + 1.
    /// CHIP-48, SCHIP 1.0:
    ///     I is then incremented by x.
    /// SCHIP-1.1, SCHIP-MODERN
    ///     I is not incremented.
    ///
    pub fn ld_vx_i(&mut self, vx: u8) {
        debug!("Fx65 - LD V{}, I", vx);
        for i in 0..(vx + 1) as u16 {
            self.v.write(i as u8, *self.ram.read(self.i + i));
        }

        if self.instruction_set == InstructionSet::Chip48
            || self.instruction_set == InstructionSet::Schip1point0
        {
            self.i = self.i + vx as u16;
        } else if self.instruction_set != InstructionSet::Schip1point1
            && self.instruction_set != InstructionSet::SchipModern
        {
            self.i = self.i + vx as u16 + 1;
        }
    }

    ///
    /// Fx75 - LD R, Vx (SCHIP Only)
    ///
    /// Store V0..VX in RPL user flags (X <= 7).
    ///
    pub fn ld_r_vx(&mut self, vx: u8) {
        debug!("Fx75 - LD R, V{}", vx);
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }
        debug!("Not implemented");
    }

    ///
    /// Fx85 - LD Vx, R (SCHIP Only)
    ///
    /// Read V0..VX from RPL user flags (X <= 7).
    ///
    pub fn ld_vx_r(&mut self, vx: u8) {
        debug!("Fx75 - LD V{}, R", vx);
        if !self.instruction_set.is_schip() {
            debug!("Only available in SCHIP mode");
            return;
        }
        debug!("Not implemented");
    }

    ///
    /// NoOp
    ///
    /// Catches unknown or invalid opcodes
    ///
    pub fn no_op(&self, opcode: Opcode) {
        debug!(
            "Unknown or invalid opcode \"{:?}\" at address 0x{}",
            opcode, self.pc.address
        );
    }
}
