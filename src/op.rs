use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref CPU_OP_CODES: Vec<OP> = vec![
        // ADC
        OP::new(0x69, Instruction::ADC, 2, 2, AddressingMode::Immediate),
        OP::new(0x65, Instruction::ADC, 2, 3, AddressingMode::ZeroPage),
        OP::new(0x75, Instruction::ADC, 2, 4, AddressingMode::ZeroPageX),
        OP::new(0x6D, Instruction::ADC, 3, 4, AddressingMode::Absolute),
        OP::new(0x7D, Instruction::ADC, 3, 4, AddressingMode::AbsoluteX),
        OP::new(0x79, Instruction::ADC, 3, 4, AddressingMode::AbsoluteY),
        OP::new(0x61, Instruction::ADC, 2, 6, AddressingMode::IndirectX),
        OP::new(0x71, Instruction::ADC, 2, 5, AddressingMode::IndirectY),

        // AND
        OP::new(0x29, Instruction::AND, 2, 2, AddressingMode::Immediate),
        OP::new(0x25, Instruction::AND, 2, 3, AddressingMode::ZeroPage),
        OP::new(0x35, Instruction::AND, 2, 4, AddressingMode::ZeroPageX),
        OP::new(0x2D, Instruction::AND, 3, 4, AddressingMode::Absolute),
        OP::new(0x3D, Instruction::AND, 3, 4, AddressingMode::AbsoluteX),
        OP::new(0x39, Instruction::AND, 3, 4, AddressingMode::AbsoluteY),
        OP::new(0x21, Instruction::AND, 2, 6, AddressingMode::IndirectX),
        OP::new(0x31, Instruction::AND, 2, 5, AddressingMode::IndirectY),

        // ASL
        OP::new(0x0A, Instruction::ASL, 1, 2, AddressingMode::Accumulator),
        OP::new(0x06, Instruction::ASL, 2, 5, AddressingMode::ZeroPage),
        OP::new(0x16, Instruction::ASL, 2, 6, AddressingMode::ZeroPageX),
        OP::new(0x0E, Instruction::ASL, 3, 6, AddressingMode::Absolute),
        OP::new(0x1E, Instruction::ASL, 3, 7, AddressingMode::AbsoluteX),

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
        code: u8,
        instruction: Instruction,
        len: u8,
        cycles: u8,
        mode: AddressingMode,
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
