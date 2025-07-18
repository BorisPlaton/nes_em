#![cfg_attr(rustfmt, rustfmt_skip)]
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCode,
    pub mode: AddressingMode,
    pub cycles: u8,
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    Indirect,
    IndexedIndirectX,
    IndirectIndexedY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
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
            AddressingMode::IndirectIndexedY => 1,
            AddressingMode::Relative => 1,
            AddressingMode::ZeroPage => 1,
            AddressingMode::ZeroPageX => 1,
            AddressingMode::ZeroPageY => 1,
            _ => 0,
        }
    }
}


lazy_static! {
    pub static ref OPCODES: HashMap<u8, Instruction> = {
        let mut opcodes = HashMap::new();

        // ADC - Add with Carry
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#ADC
        opcodes.insert(0x69, Instruction { opcode: OpCode::ADC, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0x65, Instruction { opcode: OpCode::ADC, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0x75, Instruction { opcode: OpCode::ADC, mode: AddressingMode::ZeroPageX, cycles: 2 });
        opcodes.insert(0x6D, Instruction { opcode: OpCode::ADC, mode: AddressingMode::Absolute, cycles: 3 });
        opcodes.insert(0x7D, Instruction { opcode: OpCode::ADC, mode: AddressingMode::AbsoluteX, cycles: 3 });
        opcodes.insert(0x79, Instruction { opcode: OpCode::ADC, mode: AddressingMode::AbsoluteY, cycles: 3 });
        opcodes.insert(0x61, Instruction { opcode: OpCode::ADC, mode: AddressingMode::IndexedIndirectX, cycles: 2 });
        opcodes.insert(0x71, Instruction { opcode: OpCode::ADC, mode: AddressingMode::IndirectIndexedY, cycles: 2 });

        // AND - Logical AND
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#AND
        opcodes.insert(0x29, Instruction { opcode: OpCode::AND, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0x25, Instruction { opcode: OpCode::AND, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0x35, Instruction { opcode: OpCode::AND, mode: AddressingMode::ZeroPageX, cycles: 2 });
        opcodes.insert(0x2D, Instruction { opcode: OpCode::AND, mode: AddressingMode::Absolute, cycles: 3 });
        opcodes.insert(0x3D, Instruction { opcode: OpCode::AND, mode: AddressingMode::AbsoluteX, cycles: 3 });
        opcodes.insert(0x39, Instruction { opcode: OpCode::AND, mode: AddressingMode::AbsoluteY, cycles: 3 });
        opcodes.insert(0x21, Instruction { opcode: OpCode::AND, mode: AddressingMode::IndexedIndirectX, cycles: 2 });
        opcodes.insert(0x31, Instruction { opcode: OpCode::AND, mode: AddressingMode::IndirectIndexedY, cycles: 2 });

        // ASL - Arithmetic Shift Left
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#ASL
        opcodes.insert(0x0A, Instruction { opcode: OpCode::ASL, mode: AddressingMode::Accumulator, cycles: 2 });
        opcodes.insert(0x06, Instruction { opcode: OpCode::ASL, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0x16, Instruction { opcode: OpCode::ASL, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0x0E, Instruction { opcode: OpCode::ASL, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0x1E, Instruction { opcode: OpCode::ASL, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // BCC - Branch if Carry Clear
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BCC
        opcodes.insert(0x90, Instruction { opcode: OpCode::BCC, mode: AddressingMode::Relative, cycles: 2 });

        // BCS - Branch if Carry Set
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BCS
        opcodes.insert(0xB0, Instruction { opcode: OpCode::BCS, mode: AddressingMode::Relative, cycles: 2 });

        // BEQ - Branch if Equal
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BEQ
        opcodes.insert(0xF0, Instruction { opcode: OpCode::BEQ, mode: AddressingMode::Relative, cycles: 2 });

        // BIT - BIT Test
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BIT
        opcodes.insert(0x24, Instruction { opcode: OpCode::BIT, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0x2C, Instruction { opcode: OpCode::BIT, mode: AddressingMode::Absolute, cycles: 4 });

        // BMI - Branch if Minus
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BMI
        opcodes.insert(0x30, Instruction { opcode: OpCode::BMI, mode: AddressingMode::Relative, cycles: 2 });

        // BNE - Branch if Not Equal
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BNE
        opcodes.insert(0xD0, Instruction { opcode: OpCode::BNE, mode: AddressingMode::Relative, cycles: 2 });

        // BPL - Branch if Positive
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BPL
        opcodes.insert(0x10, Instruction { opcode: OpCode::BPL, mode: AddressingMode::Relative, cycles: 2 });

        // BVC - Branch if Overflow Clear
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BVC
        opcodes.insert(0x50, Instruction { opcode: OpCode::BVC, mode: AddressingMode::Relative, cycles: 2 });

        // BVS - Branch if Overflow Set
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BVS
        opcodes.insert(0x70, Instruction { opcode: OpCode::BVS, mode: AddressingMode::Relative, cycles: 2 });

        // CLC - Clear Carry Flag
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CLC
        opcodes.insert(0x18, Instruction { opcode: OpCode::CLC, mode: AddressingMode::Implied, cycles: 2 });

        // CLD - Clear Decimal Mode
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CLD
        opcodes.insert(0xD8, Instruction { opcode: OpCode::CLD, mode: AddressingMode::Implied, cycles: 2 });

        // CLI - Clear Interrupt Disable
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CLI
        opcodes.insert(0x58, Instruction { opcode: OpCode::CLI, mode: AddressingMode::Implied, cycles: 2 });

        // CLV - Clear Overflow Flag
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CLI
        opcodes.insert(0xB8, Instruction { opcode: OpCode::CLV, mode: AddressingMode::Implied, cycles: 2 });

        // BRK - Force Interrupt
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
        opcodes.insert(0x00, Instruction { opcode: OpCode::BRK, mode: AddressingMode::Implied, cycles: 1 });

        // CMP - Compare
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CMP
        opcodes.insert(0xC9, Instruction { opcode: OpCode::CMP, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xC5, Instruction { opcode: OpCode::CMP, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0xD5, Instruction { opcode: OpCode::CMP, mode: AddressingMode::ZeroPageX, cycles: 2 });
        opcodes.insert(0xCD, Instruction { opcode: OpCode::CMP, mode: AddressingMode::Absolute, cycles: 3 });
        opcodes.insert(0xDD, Instruction { opcode: OpCode::CMP, mode: AddressingMode::AbsoluteX, cycles: 3 });
        opcodes.insert(0xD9, Instruction { opcode: OpCode::CMP, mode: AddressingMode::AbsoluteY, cycles: 3 });
        opcodes.insert(0xC1, Instruction { opcode: OpCode::CMP, mode: AddressingMode::IndexedIndirectX, cycles: 2 });
        opcodes.insert(0xD1, Instruction { opcode: OpCode::CMP, mode: AddressingMode::IndirectIndexedY, cycles: 2 });

        // CPX - Compare X Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CPX
        opcodes.insert(0xE0, Instruction { opcode: OpCode::CPX, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xE4, Instruction { opcode: OpCode::CPX, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0xEC, Instruction { opcode: OpCode::CPX, mode: AddressingMode::Absolute, cycles: 4 });

        // CPY - Compare Y Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#CPY
        opcodes.insert(0xC0, Instruction { opcode: OpCode::CPY, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xC4, Instruction { opcode: OpCode::CPY, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0xCC, Instruction { opcode: OpCode::CPY, mode: AddressingMode::Absolute, cycles: 4 });

        // DEC - Decrement Memory
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#DEC
        opcodes.insert(0xC6, Instruction { opcode: OpCode::DEC, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0xD6, Instruction { opcode: OpCode::DEC, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0xCE, Instruction { opcode: OpCode::DEC, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0xDE, Instruction { opcode: OpCode::DEC, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // DEX - Decrement X Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#DEX
        opcodes.insert(0xCA, Instruction { opcode: OpCode::DEX, mode: AddressingMode::Implied, cycles: 2 });

        // DEY - Decrement Y Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#DEY
        opcodes.insert(0x88, Instruction { opcode: OpCode::DEY, mode: AddressingMode::Implied, cycles: 2 });

        // EOR - Exclusive OR
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#EOR
        opcodes.insert(0x49, Instruction { opcode: OpCode::EOR, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0x45, Instruction { opcode: OpCode::EOR, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0x55, Instruction { opcode: OpCode::EOR, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0x4D, Instruction { opcode: OpCode::EOR, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0x5D, Instruction { opcode: OpCode::EOR, mode: AddressingMode::AbsoluteX, cycles: 4 });
        opcodes.insert(0x59, Instruction { opcode: OpCode::EOR, mode: AddressingMode::AbsoluteY, cycles: 4 });
        opcodes.insert(0x41, Instruction { opcode: OpCode::EOR, mode: AddressingMode::IndexedIndirectX, cycles: 6 });
        opcodes.insert(0x51, Instruction { opcode: OpCode::EOR, mode: AddressingMode::IndirectIndexedY, cycles: 5 });

        // INC - Increment Memory
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#INC
        opcodes.insert(0xE6, Instruction { opcode: OpCode::INC, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0xF6, Instruction { opcode: OpCode::INC, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0xEE, Instruction { opcode: OpCode::INC, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0xFE, Instruction { opcode: OpCode::INC, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // INX - Increment X Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
        opcodes.insert(0xE8, Instruction { opcode: OpCode::INX, mode: AddressingMode::Implied, cycles: 2 });

        // INY - Increment Y Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#INY
        opcodes.insert(0xC8, Instruction { opcode: OpCode::INY, mode: AddressingMode::Implied, cycles: 2 });

        // JMP - Jump
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
        opcodes.insert(0x4C, Instruction { opcode: OpCode::JMP, mode: AddressingMode::Absolute, cycles: 3 });
        opcodes.insert(0x6C, Instruction { opcode: OpCode::JMP, mode: AddressingMode::Indirect, cycles: 5 });

        // JSR - Jump to Subroutine
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#JSR
        opcodes.insert(0x20, Instruction { opcode: OpCode::JSR, mode: AddressingMode::Absolute, cycles: 6 });

        // LDA - Load Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
        opcodes.insert(0xA9, Instruction { opcode: OpCode::LDA, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xA5, Instruction { opcode: OpCode::LDA, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0xB5, Instruction { opcode: OpCode::LDA, mode: AddressingMode::ZeroPageX, cycles: 2 });
        opcodes.insert(0xAD, Instruction { opcode: OpCode::LDA, mode: AddressingMode::Absolute, cycles: 3 });
        opcodes.insert(0xBD, Instruction { opcode: OpCode::LDA, mode: AddressingMode::AbsoluteX, cycles: 3 });
        opcodes.insert(0xB9, Instruction { opcode: OpCode::LDA, mode: AddressingMode::AbsoluteY, cycles: 3 });
        opcodes.insert(0xA1, Instruction { opcode: OpCode::LDA, mode: AddressingMode::IndexedIndirectX, cycles: 2 });
        opcodes.insert(0xB1, Instruction { opcode: OpCode::LDA, mode: AddressingMode::IndirectIndexedY, cycles: 2 });

        // LDX - Load X Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#LDX
        opcodes.insert(0xA2, Instruction { opcode: OpCode::LDX, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xA6, Instruction { opcode: OpCode::LDX, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0xB6, Instruction { opcode: OpCode::LDX, mode: AddressingMode::ZeroPageY, cycles: 4 });
        opcodes.insert(0xAE, Instruction { opcode: OpCode::LDX, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0xBE, Instruction { opcode: OpCode::LDX, mode: AddressingMode::AbsoluteY, cycles: 4 });

        // LDY - Load Y Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#LDY
        opcodes.insert(0xA0, Instruction { opcode: OpCode::LDY, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xA4, Instruction { opcode: OpCode::LDY, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0xB4, Instruction { opcode: OpCode::LDY, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0xAC, Instruction { opcode: OpCode::LDY, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0xBC, Instruction { opcode: OpCode::LDY, mode: AddressingMode::AbsoluteX, cycles: 4 });

        // LSR - Logical Shift Right
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#LSR
        opcodes.insert(0x4A, Instruction { opcode: OpCode::LSR, mode: AddressingMode::Accumulator, cycles: 2 });
        opcodes.insert(0x46, Instruction { opcode: OpCode::LSR, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0x56, Instruction { opcode: OpCode::LSR, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0x4E, Instruction { opcode: OpCode::LSR, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0x5E, Instruction { opcode: OpCode::LSR, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // NOP - No Operation
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#LSR
        opcodes.insert(0xEA, Instruction { opcode: OpCode::NOP, mode: AddressingMode::Implied, cycles: 2 });

        // ORA - Logical Inclusive OR
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#ORA
        opcodes.insert(0x09, Instruction { opcode: OpCode::ORA, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0x05, Instruction { opcode: OpCode::ORA, mode: AddressingMode::ZeroPage, cycles: 2 });
        opcodes.insert(0x15, Instruction { opcode: OpCode::ORA, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0x0D, Instruction { opcode: OpCode::ORA, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0x1D, Instruction { opcode: OpCode::ORA, mode: AddressingMode::AbsoluteX, cycles: 4 });
        opcodes.insert(0x19, Instruction { opcode: OpCode::ORA, mode: AddressingMode::AbsoluteY, cycles: 4 });
        opcodes.insert(0x01, Instruction { opcode: OpCode::ORA, mode: AddressingMode::IndexedIndirectX, cycles: 5 });
        opcodes.insert(0x11, Instruction { opcode: OpCode::ORA, mode: AddressingMode::IndirectIndexedY, cycles: 6 });

        // PHA - Push Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#PHA
        opcodes.insert(0x48, Instruction { opcode: OpCode::PHA, mode: AddressingMode::Implied, cycles: 3 });

        // PHP - Push Processor Status
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#PHP
        opcodes.insert(0x08, Instruction { opcode: OpCode::PHP, mode: AddressingMode::Implied, cycles: 3 });

        // PLA - Pull Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#PLA
        opcodes.insert(0x68, Instruction { opcode: OpCode::PLA, mode: AddressingMode::Implied, cycles: 4 });

        // PLP - Pull Processor Status
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#PLP
        opcodes.insert(0x28, Instruction { opcode: OpCode::PLP, mode: AddressingMode::Implied, cycles: 4 });

        // ROL - Rotate Left
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#ROL
        opcodes.insert(0x2A, Instruction { opcode: OpCode::ROL, mode: AddressingMode::Accumulator, cycles: 2 });
        opcodes.insert(0x26, Instruction { opcode: OpCode::ROL, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0x36, Instruction { opcode: OpCode::ROL, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0x2E, Instruction { opcode: OpCode::ROL, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0x3E, Instruction { opcode: OpCode::ROL, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // ROR - Rotate Right
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#ROR
        opcodes.insert(0x6A, Instruction { opcode: OpCode::ROR, mode: AddressingMode::Accumulator, cycles: 2 });
        opcodes.insert(0x66, Instruction { opcode: OpCode::ROR, mode: AddressingMode::ZeroPage, cycles: 5 });
        opcodes.insert(0x76, Instruction { opcode: OpCode::ROR, mode: AddressingMode::ZeroPageX, cycles: 6 });
        opcodes.insert(0x6E, Instruction { opcode: OpCode::ROR, mode: AddressingMode::Absolute, cycles: 6 });
        opcodes.insert(0x7E, Instruction { opcode: OpCode::ROR, mode: AddressingMode::AbsoluteX, cycles: 7 });

        // RTI - Return from Interrupt
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#RTI
        opcodes.insert(0x40, Instruction { opcode: OpCode::RTI, mode: AddressingMode::Implied, cycles: 6 });

        // RTS - Return from Subroutine
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#RTS
        opcodes.insert(0x60, Instruction { opcode: OpCode::RTS, mode: AddressingMode::Implied, cycles: 6 });

        // SBC - Subtract with Carry
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#SBC
        opcodes.insert(0xE9, Instruction { opcode: OpCode::SBC, mode: AddressingMode::Immediate, cycles: 2 });
        opcodes.insert(0xE5, Instruction { opcode: OpCode::SBC, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0xF5, Instruction { opcode: OpCode::SBC, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0xED, Instruction { opcode: OpCode::SBC, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0xFD, Instruction { opcode: OpCode::SBC, mode: AddressingMode::AbsoluteX, cycles: 4 });
        opcodes.insert(0xF9, Instruction { opcode: OpCode::SBC, mode: AddressingMode::AbsoluteY, cycles: 4 });
        opcodes.insert(0xE1, Instruction { opcode: OpCode::SBC, mode: AddressingMode::IndexedIndirectX, cycles: 5 });
        opcodes.insert(0xF1, Instruction { opcode: OpCode::SBC, mode: AddressingMode::IndirectIndexedY, cycles: 6 });

        // SEC - Set Carry Flag
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#SEC
        opcodes.insert(0x38, Instruction { opcode: OpCode::SEC, mode: AddressingMode::Implied, cycles: 2 });

        // SED - Set Decimal Flag
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#SED
        opcodes.insert(0xF8, Instruction { opcode: OpCode::SED, mode: AddressingMode::Implied, cycles: 2 });

        // SEI - Set Interrupt Disable
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#SEI
        opcodes.insert(0x78, Instruction { opcode: OpCode::SEI, mode: AddressingMode::Implied, cycles: 2 });

        // STA - Store Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#STA
        opcodes.insert(0x85, Instruction { opcode: OpCode::STA, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0x95, Instruction { opcode: OpCode::STA, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0x8D, Instruction { opcode: OpCode::STA, mode: AddressingMode::Absolute, cycles: 4 });
        opcodes.insert(0x9D, Instruction { opcode: OpCode::STA, mode: AddressingMode::AbsoluteX, cycles: 5 });
        opcodes.insert(0x99, Instruction { opcode: OpCode::STA, mode: AddressingMode::AbsoluteY, cycles: 5 });
        opcodes.insert(0x81, Instruction { opcode: OpCode::STA, mode: AddressingMode::IndexedIndirectX, cycles: 6 });
        opcodes.insert(0x91, Instruction { opcode: OpCode::STA, mode: AddressingMode::IndirectIndexedY, cycles: 6 });

        // STX - Store X Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#STX
        opcodes.insert(0x86, Instruction { opcode: OpCode::STX, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0x96, Instruction { opcode: OpCode::STX, mode: AddressingMode::ZeroPageY, cycles: 4 });
        opcodes.insert(0x8E, Instruction { opcode: OpCode::STX, mode: AddressingMode::Absolute, cycles: 4 });

        // STY - Store Y Register
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#STY
        opcodes.insert(0x84, Instruction { opcode: OpCode::STY, mode: AddressingMode::ZeroPage, cycles: 3 });
        opcodes.insert(0x94, Instruction { opcode: OpCode::STY, mode: AddressingMode::ZeroPageX, cycles: 4 });
        opcodes.insert(0x8C, Instruction { opcode: OpCode::STY, mode: AddressingMode::Absolute, cycles: 4 });

        // TAX - Transfer Accumulator to X
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
        opcodes.insert(0xAA, Instruction { opcode: OpCode::TAX, mode: AddressingMode::Implied, cycles: 2 });

        // TAY - Transfer Accumulator to Y
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAY
        opcodes.insert(0xA8, Instruction { opcode: OpCode::TAY, mode: AddressingMode::Implied, cycles: 2 });

        // TSX - Transfer Stack Pointer to X
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TSX
        opcodes.insert(0xBA, Instruction { opcode: OpCode::TSX, mode: AddressingMode::Implied, cycles: 2 });

        // TXA - Transfer X to Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TXA
        opcodes.insert(0x8A, Instruction { opcode: OpCode::TXA, mode: AddressingMode::Implied, cycles: 2 });

        // TXS - Transfer X to Stack Pointer
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TXS
        opcodes.insert(0x9A, Instruction { opcode: OpCode::TXS, mode: AddressingMode::Implied, cycles: 2 });

        // TYA - Transfer Y to Accumulator
        // https://www.nesdev.org/obelisk-6502-guide/reference.html#TYA
        opcodes.insert(0x98, Instruction { opcode: OpCode::TYA, mode: AddressingMode::Implied, cycles: 2 });

        opcodes
    };
}
