use super::error::UnknownOpCode;
use crate::cpu::codes::OPCODES;

pub struct Instruction {
    pub opcode: OpCode,
    pub mode: AddressingMode,
    pub cycles: u8,
}

pub enum OpCode {
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

pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Indirect,
    IndexedIndirectX,
    IndexedIndirectY,
    IndirectIndexedX,
    IndirectIndexedY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Implied,
}

impl AddressingMode {
    pub fn operand_bytes(&self) -> u8 {
        match self {
            AddressingMode::Absolute => 2,
            AddressingMode::AbsoluteX => 2,
            AddressingMode::AbsoluteY => 2,
            AddressingMode::Immediate => 1,
            AddressingMode::Indirect => 2,
            AddressingMode::IndexedIndirectX => 1,
            AddressingMode::IndexedIndirectY => 1,
            AddressingMode::ZeroPage => 1,
            AddressingMode::ZeroPageX => 1,
            AddressingMode::ZeroPageY => 1,
            AddressingMode::IndirectIndexedX => 1,
            AddressingMode::IndirectIndexedY => 1,
            AddressingMode::Relative => 1,
            _ => 0,
        }
    }
}

impl TryFrom<u8> for Instruction {
    type Error = UnknownOpCode;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let instruction = OPCODES.get(&value).ok_or(UnknownOpCode(value))?;
    }
}
