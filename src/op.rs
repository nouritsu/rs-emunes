use lazy_static::lazy_static;
use std::collections::HashMap;
use {AddressingMode::*, Instruction::*};

lazy_static! {

    static ref CPU_OP_CODES: Vec<OP> = vec![
        // ADd with Carry
        OP::new(ADC, Immediate, 0x69, 2, 2),
        OP::new(ADC, ZeroPage, 0x65, 2, 3),
        OP::new(ADC, ZeroPageX, 0x75, 2, 4),
        OP::new(ADC, Absolute, 0x6D, 3, 4),
        OP::new(ADC, AbsoluteX, 0x7D, 3, 4 /* +1 if page crossed */),
        OP::new(ADC, AbsoluteY, 0x79, 3, 4 /* +1 if page crossed */),
        OP::new(ADC, IndirectX, 0x61, 2, 6),
        OP::new(ADC, IndirectY, 0x71, 2, 5 /* +1 if page crossed */),

        // AND
        OP::new(AND, Immediate, 0x29, 2, 2),
        OP::new(AND, ZeroPage, 0x25, 2, 3),
        OP::new(AND, ZeroPageX, 0x35, 2, 4),
        OP::new(AND, Absolute, 0x2D, 3, 4),
        OP::new(AND, AbsoluteX, 0x3D, 3, 4 /* +1 if page crossed */),
        OP::new(AND, AbsoluteY, 0x39, 3, 4 /* +1 if page crossed */),
        OP::new(AND, IndirectX, 0x21, 2, 6),
        OP::new(AND, IndirectY, 0x31, 2, 5 /* +1 if page crossed */),

        // Arithmetic Shift Left
        OP::new(ASL, Accumulator, 0x0A, 1, 2),
        OP::new(ASL, ZeroPage, 0x06, 2, 5),
        OP::new(ASL, ZeroPageX, 0x16, 2, 6),
        OP::new(ASL, Absolute, 0x0E, 3, 6),
        OP::new(ASL, AbsoluteX, 0x1E, 3, 7),

        // Branch if Carry Clear
        OP::new(BCC, Relative, 0x90, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if Carry Set
        OP::new(BCS, Relative, 0xB0, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if EQual
        OP::new(BEQ, Relative, 0xF0, 2, 2 /* +1 if branched, +2 if to a new page */),

        // BIt Test
        OP::new(BIT, ZeroPage, 0x24, 2, 3),
        OP::new(BIT, Absolute, 0x2C, 3, 4),

        // Branch if MInus
        OP::new(BMI, Relative, 0x30, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if Not Equal
        OP::new(BNE, Relative, 0xD0, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if Positive
        OP::new(BPL, Relative, 0x10, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Force Interrupt
        OP::new(BRK, NoneAddressing, 0x00, 1, 7),

        // Branch if Overflow Clear
        OP::new(BVC, Relative, 0x50, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if Overflow Set
        OP::new(BVS, Relative, 0x70, 2, 2 /* +1 if branched, +2 if to a new page */),

    ];

    pub static ref OPCODES: HashMap<u8, &'static OP> = {
        let mut hm = HashMap::new();

        for op in &*CPU_OP_CODES  {
            hm.insert(op.code, op);
        }

        hm
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OP {
    pub code: u8,
    pub instruction: Instruction,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OP {
    pub fn new(
        instruction: Instruction,
        mode: AddressingMode,
        code: u8,
        len: u8,
        cycles: u8,
    ) -> Self {
        Self {
            code,
            instruction,
            len,
            cycles,
            mode,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDS,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Accumulator,
    NoneAddressing,
}
