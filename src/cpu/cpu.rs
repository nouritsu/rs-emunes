use super::{
    mem::Mem,
    op::{AddressingMode, OPCODES},
};
use bitflags::bitflags;

bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CPUFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIVE          = 0b10000000;
    }
}

#[derive(Debug)]
pub struct CPU {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub status: u8,
    pub pc: u16,
    pub memory: [u8; 0xFFFF],
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
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

    fn get_address(&self, mode: &AddressingMode) -> u16 {
        use AddressingMode::*;

        match mode {
            Immediate => self.pc,
            ZeroPage => self.mem_read(self.pc) as u16,
            ZeroPageX => self.mem_read(self.pc).wrapping_add(self.x) as u16,
            ZeroPageY => self.mem_read(self.pc).wrapping_add(self.y) as u16,
            Absolute => self.mem_read_u16(self.pc),
            AbsoluteX => self.mem_read_u16(self.pc).wrapping_add(self.x as u16),
            AbsoluteY => self.mem_read_u16(self.pc).wrapping_add(self.y as u16),
            IndirectX => {
                let base = self.mem_read(self.pc);

                let ptr = (base as u8).wrapping_add(self.x);
                let lo = self.mem_read(ptr as u16) as u16;
                let hi = self.mem_read(ptr.wrapping_add(1) as u16) as u16;

                hi << 8 | lo
            }
            IndirectY => {
                let base = self.mem_read(self.pc);

                let lo = self.mem_read(base as u16) as u16;
                let hi = self.mem_read((base as u8).wrapping_add(1) as u16) as u16;

                (hi << 8 | lo).wrapping_add(self.y as u16)
            }
            NoneAddressing => panic!("attempted to get address from NoneAddressing"),
        }
    }
}
