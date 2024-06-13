///
/// Stack
///
#[derive(Debug)]
pub struct Stack {
    data: Box<[u16; 16]>,
    pointer: u8,
}

impl Stack {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: Box::new([0; 16]),
            pointer: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: u16) {
        self.pointer += 1;
        self.data[self.pointer as usize] = value;
    }

    #[inline]
    pub fn pop(&mut self) -> u16 {
        let addr = self.data[self.pointer as usize];
        self.pointer -= 1;

        addr
    }

    #[inline]
    pub fn clear(&mut self) {
        self.data = Box::new([0; 16]);
        self.pointer = 0;
    }
}
