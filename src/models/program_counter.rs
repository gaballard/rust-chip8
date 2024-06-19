use crate::constants;

///
/// Program Counter
///
#[derive(Debug)]
pub struct ProgramCounter {
    pub address: u16,
}

impl ProgramCounter {
    #[inline]
    pub const fn new() -> Self {
        ProgramCounter {
            address: constants::PROGRAM_START_ADDR,
        }
    }

    #[inline]
    pub fn next(&mut self) {
        self.address += constants::OPCODE_SIZE;
    }

    #[inline]
    pub fn jump(&mut self, jump_addr: u16) {
        self.address = jump_addr;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.address = constants::PROGRAM_START_ADDR;
    }
}
