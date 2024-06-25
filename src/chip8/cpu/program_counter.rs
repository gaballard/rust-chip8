use crate::constants;

///
/// Program Counter
///
#[derive(Debug)]
pub struct ProgramCounter {
    program_start_addr: u16,
    opcode_size: u16,
    pub address: u16,
}

impl Default for ProgramCounter {
    fn default() -> Self {
        Self {
            program_start_addr: constants::PROGRAM_START_ADDR,
            opcode_size: constants::OPCODE_SIZE,
            address: constants::PROGRAM_START_ADDR,
        }
    }
}

impl ProgramCounter {
    #[inline]
    pub fn next(&mut self) {
        self.address += self.opcode_size;
    }

    #[inline]
    pub fn jump(&mut self, jump_addr: u16) {
        self.address = jump_addr;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.address = self.program_start_addr;
    }
}
