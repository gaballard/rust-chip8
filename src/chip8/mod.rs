mod cpu;
mod memory;
mod video;

pub use cpu::Cpu;
pub use memory::Memory;
pub use video::VideoMemory;

pub trait Storage {
    fn read(&self, addr: u16) -> &u8;

    fn write(&mut self, addr: u16, value: u8);

    fn clear(&mut self);
}
