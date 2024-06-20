use crate::{
    components::{Memory, Opcode},
    constants,
    devices::Tape,
    models::ProgramCounter,
};

///
/// Decompiler
///
/// Converts bytecode to assembly.
///
pub struct Decompiler {
    tape: Tape,
    pc: ProgramCounter,
    ram: Memory,
}

impl Decompiler {
    pub fn new() -> Self {
        Self {
            tape: Tape::new(),
            pc: ProgramCounter::new(),
            ram: Memory::new(),
        }
    }

    pub fn load_into_ram(&mut self, filename: &String) -> u32 {
        self.tape.load_rom(filename);

        let mut address = constants::PROGRAM_START_ADDR;
        for v in &self.tape.rom {
            self.ram.write(address, *v);
            address += 1;
        }

        self.tape.rom.len().try_into().unwrap()
    }

    pub fn decompile(&mut self, input_filename: &String, output_filename: &String) {
        let rom_length = self.load_into_ram(input_filename);

        let mut output: Vec<String> = Vec::new();

        for _ in 0..rom_length {
            let curr_addr = self.pc.address;

            let instruction =
                (*self.ram.read(curr_addr) as u16) << 8 | (*self.ram.read(curr_addr + 1) as u16);

            self.pc.next();

            let opcode: Opcode = (
                ((instruction & 0xF000) >> 12) as u8,
                ((instruction & 0x0F00) >> 8) as u8,
                ((instruction & 0x00F0) >> 4) as u8,
                (instruction & 0x000F) as u8,
            );

            let x: u8 = opcode.1.try_into().unwrap();
            let y: u8 = opcode.2.try_into().unwrap();

            let n: u8 = opcode.3.try_into().unwrap();
            let kk = (instruction & 0x00FF) as u8;
            let nnn = instruction & 0x0FFF;

            let command = format!(
                "${:x} {}",
                curr_addr,
                match opcode {
                    (0x0, 0x0, 0xC, _) => format!("SCD {:x}", n), // SCHIP only
                    (0x0, 0x0, 0xE, 0x0) => "CLS".to_string(),
                    (0x0, 0x0, 0xE, 0xE) => "RET".to_string(),
                    (0x0, 0x0, 0xF, 0xB) => "SCR".to_string(), // SCHIP only
                    (0x0, 0x0, 0xF, 0xC) => "SCL".to_string(), // SCHIP only
                    (0x0, 0x0, 0xF, 0xD) => "EXIT".to_string(), // SCHIP only
                    (0x0, 0x0, 0xF, 0xE) => "LOW".to_string(), // SCHIP only
                    (0x0, 0x0, 0xF, 0xF) => "HIGH".to_string(), // SCHIP only
                    (0x1, _, _, _) => format!("JP ${:x}", nnn),
                    (0x2, _, _, _) => format!("CALL ${:x}", nnn),
                    (0x3, _, _, _) => format!("SE V{}, 0x{:x}", x, kk),
                    (0x4, _, _, _) => format!("SNE V{}, 0x{:x}", x, kk),
                    (0x5, _, _, 0x0) => format!("SE V{}, V{}", x, y),
                    (0x6, _, _, _) => format!("LD V{}, 0x{:x}", x, kk),
                    (0x7, _, _, _) => format!("ADD V{}, 0x{:x}", x, kk),
                    (0x8, _, _, 0x0) => format!("LD V{}, V{}", x, y),
                    (0x8, _, _, 0x1) => format!("OR V{}, V{}", x, y),
                    (0x8, _, _, 0x2) => format!("AND V{}, V{}", x, y),
                    (0x8, _, _, 0x3) => format!("XOR V{}, V{}", x, y),
                    (0x8, _, _, 0x4) => format!("ADD V{}, V{}", x, y),
                    (0x8, _, _, 0x5) => format!("SUB V{}, V{}", x, y),
                    (0x8, _, _, 0x6) => format!("SHR V{}, V{}", x, y), // SCHIP behavior
                    (0x8, _, _, 0x7) => format!("SUBN V{}, V{}", x, y),
                    (0x8, _, _, 0xE) => format!("SHL V{}, V{}", x, y), // SCHIP behavior
                    (0x9, _, _, 0x0) => format!("SNE V{}, V{}", x, y),
                    (0xA, _, _, _) => format!("LD I, ${:x}", nnn),
                    (0xB, _, _, _) => format!("JP V0, ${:x}", nnn),
                    (0xC, _, _, _) => format!("RND V{}, 0x{:x}", x, kk),
                    (0xD, _, _, _) => format!("DRW V{}, V{}, 0x{:x}", x, y, n), // SCHIP behavior
                    (0xE, _, 0x9, 0xE) => format!("SKP V{}", x),
                    (0xE, _, 0xA, 0x1) => format!("SKNP V{}", x),
                    (0xF, _, 0x0, 0x7) => format!("LD V{}, DT", x),
                    (0xF, _, 0x0, 0xA) => format!("LD V{}, K", x),
                    (0xF, _, 0x1, 0x5) => format!("LD DT, V{}", x),
                    (0xF, _, 0x1, 0x8) => format!("LD ST, V{}", x),
                    (0xF, _, 0x1, 0xE) => format!("ADD I, V{}", x),
                    (0xF, _, 0x2, 0x9) => format!("LD F, V{}", x),
                    (0xF, _, 0x3, 0x0) => format!("LD HF, V{}", x), // SCHIP only
                    (0xF, _, 0x3, 0x3) => format!("LD B, V{}", x),
                    (0xF, _, 0x5, 0x5) => format!("LD I, V{}", x), // SCHIP behavior
                    (0xF, _, 0x6, 0x5) => format!("LD V{}, I", x),
                    (0xF, _, 0x7, 0x5) => format!("LD R, V{}", x), // SCHIP only
                    (0xF, _, 0x8, 0x5) => format!("LD V{}, R", x), // SCHIP only
                    _ => format!("0x{:x}", instruction),
                }
            );

            output.push(command);
        }

        let output_data = output.join("\n");

        self.tape.write(output_filename, output_data);
    }
}
