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

        // force interrupt
        OP::new(BRK, NoneAddressing, 0x00, 1, 7),

        // Branch if oVerflow Clear
        OP::new(BVC, Relative, 0x50, 2, 2 /* +1 if branched, +2 if to a new page */),

        // Branch if oVerflow Set
        OP::new(BVS, Relative, 0x70, 2, 2 /* +1 if branched, +2 if to a new page */),

        // CLear Carry flag
        OP::new(CLC, NoneAddressing, 0x18, 1, 2),

        // CLear Decimal mode
        OP::new(CLD, NoneAddressing, 0xD8, 1, 2),

        // CLear Interrupt disable
        OP::new(CLI, NoneAddressing, 0x58, 1, 2),

        // CLear oVerflow flag
        OP::new(CLV, NoneAddressing, 0xB8, 1, 2),

        // CoMPare
        OP::new(CMP, Immediate, 0xC9, 2, 2),
        OP::new(CMP, ZeroPage, 0xC5, 2, 3),
        OP::new(CMP, ZeroPageX, 0xD5, 2, 4),
        OP::new(CMP, Absolute, 0xCD, 3, 4),
        OP::new(CMP, AbsoluteX, 0xDD, 3, 4 /* +1 if page crossed */),
        OP::new(CMP, AbsoluteY, 0xD9, 3, 4 /* +1 if page crossed */),
        OP::new(CMP, IndirectX, 0xC1, 2, 6),
        OP::new(CMP, IndirectY, 0xD1, 2, 5 /* +1 if page crossed */),

        // ComPare X register
        OP::new(CPX, Immediate, 0xE0, 2, 2),
        OP::new(CPX, ZeroPage, 0xE4, 2, 3),
        OP::new(CPX, Absolute, 0xEC, 3, 4),

        // ComPare Y register
        OP::new(CPY, Immediate, 0xC0, 2, 2),
        OP::new(CPY, ZeroPage, 0xC4, 2, 3),
        OP::new(CPY, Absolute, 0xCC, 3, 4),

        // DECrement Memory
        OP::new(DEC, ZeroPage, 0xC6, 2, 5),
        OP::new(DEC, ZeroPageX, 0xD6, 2, 6),
        OP::new(DEC, Absolute, 0xCE, 3, 6),
        OP::new(DEC, AbsoluteX, 0xDE, 3, 7),

        // DEcrement X register
        OP::new(DEX, NoneAddressing, 0xCA, 1, 2),

        // DEcrement Y register
        OP::new(DEY, NoneAddressing, 0x88, 1, 2),

        // Exclusive OR
        OP::new(EOR, Immediate, 0x49, 2, 2),
        OP::new(EOR, ZeroPage, 0x45, 2, 3),
        OP::new(EOR, ZeroPageX, 0x55, 2, 4),
        OP::new(EOR, Absolute, 0x4D, 3, 4),
        OP::new(EOR, AbsoluteX, 0x5D, 3, 4 /* +1 if page crossed */),
        OP::new(EOR, AbsoluteY, 0x59, 3, 4 /* +1 if page crossed */),
        OP::new(EOR, IndirectX, 0x41, 2, 6),
        OP::new(EOR, IndirectY, 0x51, 2, 5 /* +1 if page crossed */),

        // INCrement memory
        OP::new(INC, ZeroPage, 0xE6, 2, 5),
        OP::new(INC, ZeroPageX, 0xF6, 2, 6),
        OP::new(INC, Absolute, 0xEE, 3, 6),
        OP::new(INC, AbsoluteX, 0xFE, 3, 7),

        // INcrement X register
        OP::new(INX, NoneAddressing, 0xE8, 1, 2),

        // INcrement Y register
        OP::new(INY, NoneAddressing, 0xC8, 1, 2),

        // JuMP
        OP::new(JMP, Absolute, 0x4C, 3, 3),
        OP::new(JMP, Indirect, 0x6C, 3, 5),

        // Jump to SubRoutine
        OP::new(JSR, Absolute, 0x20, 3, 6),

        // LoaD Accumulator
        OP::new(LDA, Immediate, 0xA9, 2, 2),
        OP::new(LDA, ZeroPage, 0xA5, 2, 3),
        OP::new(LDA, ZeroPageX, 0xB5, 2, 4),
        OP::new(LDA, Absolute, 0xAD, 3, 4),
        OP::new(LDA, AbsoluteX, 0xBD, 3, 4 /* +1 if page crossed */),
        OP::new(LDA, AbsoluteY, 0xB9, 3, 4 /* +1 if page crossed */),
        OP::new(LDA, IndirectX, 0xA1, 2, 6),
        OP::new(LDA, IndirectY, 0xA9, 2, 5 /* +1 if page crossed */),

        // LoaD X register
        OP::new(LDX, Immediate, 0xA2, 2, 2),
        OP::new(LDX, ZeroPage, 0xA6, 2, 3),
        OP::new(LDX, ZeroPageY, 0xB6, 2, 4),
        OP::new(LDX, Absolute, 0xAE, 3, 4),
        OP::new(LDX, AbsoluteY, 0xA2, 3, 4 /* +1 if page crossed */),

        // LoaD Y register
        OP::new(LDY, Immediate, 0xA0, 2, 2),
        OP::new(LDY, ZeroPage, 0xA4, 2, 3),
        OP::new(LDY, ZeroPageY, 0xB4, 2, 4),
        OP::new(LDY, Absolute, 0xAC, 3, 4),
        OP::new(LDY, AbsoluteY, 0xBC, 3, 4 /* +1 if page crossed */),

        // Logical Shift Right
        OP::new(LSR, Accumulator, 0x4A, 1, 2),
        OP::new(LSR, ZeroPage, 0x46, 2, 5),
        OP::new(LSR, ZeroPageX, 0x56, 2, 6),
        OP::new(LSR, Absolute, 0x4E, 3, 6),
        OP::new(LSR, AbsoluteY, 0x5E, 3, 7),

        // No OPeration
        OP::new(NOP, NoneAddressing, 0xEA, 1, 2),
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
    LDX,
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
    Indirect,
    IndirectX,
    IndirectY,
    Accumulator,
    NoneAddressing,
}
