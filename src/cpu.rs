#[derive(Debug)]
pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    pub pc: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            status: 0,
            pc: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn interpret(&mut self, program: &[u8]) {
        todo!();
    }
}
