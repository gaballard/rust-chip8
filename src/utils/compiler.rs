use crate::{components::Memory, devices::Tape, models::ProgramCounter};

///
/// Compiler
///
/// Converts assembly to bytecode.
///
pub struct _Compiler {
    tape: Tape,
    pc: ProgramCounter,
    ram: Memory,
}

impl _Compiler {
    pub fn _new() -> Self {
        Self {
            tape: Tape::new(),
            pc: ProgramCounter::new(),
            ram: Memory::new(),
        }
    }

    pub fn _load_into_ram(&mut self, filename: &String) -> u32 {
        self.tape.load_rom(filename);

        let mut address = 0;
        for v in &self.tape.rom {
            self.ram.write(address, *v);
            address += 1;
        }

        self.tape.rom.len().try_into().unwrap()
    }

    pub fn _compile(&mut self, _input_filename: &String, _output_filename: &String) {}
}
