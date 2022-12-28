///
/// Stack
///
#[derive(Debug)]
pub struct Stack {
    data: Box<[u16; 16]>,
    pointer: u8,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            data: Box::new([0; 16]),
            pointer: 0,
        }
    }

    pub fn push(&mut self, value: u16) {
        self.pointer += 1;
        self.data[self.pointer as usize] = value;
    }

    pub fn pop(&mut self) -> u16 {
        let addr = self.data[self.pointer as usize];
        self.pointer -= 1;

        addr
    }

    pub fn clear(&mut self) {
        self.data = Box::new([0; 16]);
        self.pointer = 0;
    }
}
