use std::fs;

use crate::{components::Memory, constants, devices::Tape, models::ProgramCounter};

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

    pub fn _compile(&mut self, _input_filename: &String, _output_filename: &String) {
        let path = format!("{}/{}", constants::ROM_FOLDER, _input_filename);
        let assembly_data = fs::read_to_string(&path)
            .expect(format!("File does not exist at path {}", path).as_str());

        let mut _output: Vec<u8> = Vec::new();

        for assembly_line in assembly_data.split("\n") {
            let mut line = assembly_line.split(" ");
            let mut _addr = line.nth(0).unwrap();
            let mut _command = line.nth(1).unwrap();

            for line_element in assembly_line.split(" ") {
                if line_element.chars().nth(0).unwrap() == '$' {
                    _addr = line_element;
                }
            }
        }
    }
}
