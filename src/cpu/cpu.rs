use crate::cpu::error::{StackError, UnknownOpCode};
use crate::cpu::opcode::OPCODES;
use crate::cpu::opcode::{AddressingMode, Instruction, OpCode};
use crate::cpu::register::counter::ProgramCounter;
use crate::cpu::register::register::Register;
use crate::cpu::register::stack::Stack;
use crate::cpu::register::status::Status;
use crate::mem::map::{IOOperation, MemoryMap};
use std::error::Error;

pub struct CPU {
    accumulator: Register<u8>,
    register_x: Register<u8>,
    register_y: Register<u8>,
    program_counter: ProgramCounter,
    status: Status,
    memory: MemoryMap,
    stack: Stack,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: Register::new(0),
            register_x: Register::new(0),
            register_y: Register::new(0),
            program_counter: ProgramCounter::new(),
            status: Status::new(),
            memory: MemoryMap::new(),
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        // TODO: Add proper handling of errors that may occur
        self.interpret().unwrap();
    }

    fn load_program(&mut self, program: Vec<u8>) {
        self.memory.copy_to(0x8000, &program);
        self.memory.write(0xFFFC, 0x8000u16);
    }

    fn reset(&mut self) {
        self.program_counter.set(self.memory.read(0xFFFC));
        self.accumulator.set(0);
        self.register_x.set(0);
        self.status.reset();
    }

    fn interpret(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let instruction = self.next_instruction()?;
            match instruction.opcode {
                OpCode::ADC => self.adc(&instruction.mode),
                OpCode::AND => self.and(&instruction.mode),
                OpCode::ASL => self.asl(&instruction.mode),
                OpCode::BCC => self.bcc(&instruction.mode),
                OpCode::BCS => self.bcs(&instruction.mode),
                OpCode::BEQ => self.beq(&instruction.mode),
                OpCode::BIT => self.bit(&instruction.mode),
                OpCode::BMI => self.bmi(&instruction.mode),
                OpCode::BNE => self.bne(&instruction.mode),
                OpCode::BPL => self.bpl(&instruction.mode),
                OpCode::BRK => return Ok(()),
                OpCode::BVC => self.bvc(&instruction.mode),
                OpCode::BVS => self.bvs(&instruction.mode),
                OpCode::CLC => self.clc(),
                OpCode::CLD => self.cld(),
                OpCode::CLI => self.cli(),
                OpCode::CLV => self.clv(),
                OpCode::CMP => self.cmp(&instruction.mode),
                OpCode::CPX => self.cpx(&instruction.mode),
                OpCode::CPY => self.cpy(&instruction.mode),
                OpCode::DEC => self.dec(&instruction.mode),
                OpCode::DEX => self.dex(),
                OpCode::DEY => self.dey(),
                OpCode::EOR => self.eor(&instruction.mode),
                OpCode::INC => self.inc(&instruction.mode),
                OpCode::INX => self.inx(),
                OpCode::INY => self.iny(),
                OpCode::JMP => self.jmp(&instruction.mode),
                OpCode::JSR => self.jsr(&instruction.mode)?,
                OpCode::LDA => self.lda(&instruction.mode),
                OpCode::LDX => self.ldx(&instruction.mode),
                OpCode::LDY => self.ldy(&instruction.mode),
                OpCode::LSR => self.lsr(&instruction.mode),
                OpCode::NOP => self.nop(),
                OpCode::ORA => self.ora(&instruction.mode),
                OpCode::PHA => self.pha()?,
                OpCode::PHP => self.php()?,
                OpCode::PLA => self.pla()?,
                OpCode::PLP => self.plp()?,
                OpCode::ROL => self.rol(&instruction.mode),
                OpCode::ROR => self.ror(&instruction.mode),
                OpCode::RTI => self.rti()?,
                OpCode::RTS => self.rts()?,
                OpCode::SBC => self.sbc(&instruction.mode),
                OpCode::SEC => self.sec(),
                OpCode::SED => self.sed(),
                OpCode::SEI => self.sei(),
                OpCode::STA => self.sta(&instruction.mode),
                OpCode::STX => self.stx(&instruction.mode),
                OpCode::STY => self.sty(&instruction.mode),
                OpCode::TAX => self.tax(),
                OpCode::TAY => self.tay(),
                OpCode::TSX => self.tsx(),
                OpCode::TXA => self.txa(),
                OpCode::TXS => self.txs()?,
                OpCode::TYA => self.tya(),
            };
        }
    }

    fn adc(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        self.adc_operation(value);
    }

    // Moved ADC instruction's logic to separate function, because the same logic
    // is reused in the SBC instruction.
    fn adc_operation(&mut self, value: u8) {
        let (result, is_carry_flag_set) = self
            .accumulator
            .add(value + self.status.is_carry_flag_set() as u8);
        self.status.set_carry_flag_to(is_carry_flag_set);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        self.status
            .set_overflow_flag_to((value ^ result) & (result ^ self.accumulator.get()) & 0x80 != 0);
        self.accumulator.set(result);
    }

    fn and(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let result = value & self.accumulator.get();
        self.accumulator.set(result);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
    }

    fn asl(&mut self, addressing_mode: &AddressingMode) {
        let (old_value, shifted_value) = match addressing_mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = old_value << 1;
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let old_value_address = self.get_address(addressing_mode);
                let old_value: u8 = self.memory.read(old_value_address);
                let shifted_value = old_value << 1;
                self.memory.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 0b1000_0000 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
    }

    fn bcc(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if !self.status.is_carry_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bcs(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if self.status.is_carry_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn beq(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if self.status.is_zero_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bit(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let result = value & self.accumulator.get();
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(value);
        self.status.set_overflow_flag_to(value & 0b0100_0000 != 0);
    }

    fn bmi(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if self.status.is_negative_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bne(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if !self.status.is_zero_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bpl(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if !self.status.is_negative_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bvc(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if !self.status.is_overflow_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn bvs(&mut self, addressing_mode: &AddressingMode) {
        let offset = self.get_value(addressing_mode);
        if self.status.is_overflow_flag_set() {
            self.program_counter.move_with_offset(offset);
        }
    }

    fn clc(&mut self) {
        self.status.set_carry_flag_to(false);
    }

    fn cld(&mut self) {
        self.status.set_decimal_mode_flag_to(false);
    }

    fn cli(&mut self) {
        self.status.set_interrupt_disable_flag_to(false);
    }

    fn clv(&mut self) {
        self.status.set_overflow_flag_to(false);
    }

    fn cmp(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let accumulator_value = self.accumulator.get();
        let result = self.accumulator.sub(value, false);
        self.status.set_carry_flag_to(accumulator_value >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
    }

    fn cpx(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let register_x_value = self.register_x.get();
        let result = self.register_x.sub(value, false);
        self.status.set_carry_flag_to(register_x_value >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
    }

    fn cpy(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let register_y_value = self.register_y.get();
        let result = self.register_y.sub(value, false);
        self.status.set_carry_flag_to(register_y_value >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
    }

    fn dec(&mut self, addressing_mode: &AddressingMode) {
        let value_address = self.get_address(addressing_mode);
        let value: u8 = self.memory.read(value_address);
        let new_value = value.wrapping_sub(1);
        self.memory.write(value_address, new_value);
        self.status.set_zero_flag(new_value);
        self.status.set_negative_flag(new_value);
    }

    fn dex(&mut self) {
        self.register_x.dec();
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
    }

    fn dey(&mut self) {
        self.register_y.dec();
        self.status.set_zero_flag(self.register_y.get());
        self.status.set_negative_flag(self.register_y.get());
    }

    fn eor(&mut self, addressing_mode: &AddressingMode) {
        let new_value = self.accumulator.get() ^ self.get_value(addressing_mode);
        self.accumulator.set(new_value);
        self.status.set_zero_flag(new_value);
        self.status.set_negative_flag(new_value);
    }

    fn inx(&mut self) {
        let new_register_value = self.register_x.inc();
        self.status.set_zero_flag(new_register_value);
        self.status.set_negative_flag(new_register_value);
    }

    fn inc(&mut self, addressing_mode: &AddressingMode) {
        let value_address = self.get_address(addressing_mode);
        let value: u8 = self.memory.read(value_address);
        let new_value = value.wrapping_add(1);
        self.memory.write(value_address, new_value);
        self.status.set_zero_flag(new_value);
        self.status.set_negative_flag(new_value);
    }

    fn iny(&mut self) {
        let new_register_value = self.register_y.inc();
        self.status.set_zero_flag(new_register_value);
        self.status.set_negative_flag(new_register_value);
    }

    fn jmp(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address(addressing_mode);
        self.program_counter.set(address);
    }

    fn jsr(&mut self, addressing_mode: &AddressingMode) -> Result<(), StackError> {
        let address = self.get_address(addressing_mode);
        let current_address_bytes: [u8; 2] =
            self.program_counter.get().wrapping_sub(1).to_be_bytes();
        self.stack
            .push(current_address_bytes[0], &mut self.memory)?;
        self.stack
            .push(current_address_bytes[1], &mut self.memory)?;
        self.program_counter.set(address);
        Ok(())
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
    }

    fn ldx(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        self.register_x.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
    }

    fn ldy(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        self.register_y.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
    }

    fn lsr(&mut self, addressing_mode: &AddressingMode) {
        let (old_value, shifted_value) = match addressing_mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = old_value >> 1;
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let old_value_address = self.get_address(addressing_mode);
                let old_value: u8 = self.memory.read(old_value_address);
                let shifted_value = old_value >> 1;
                self.memory.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 1 != 0);
        self.status.set_negative_flag(0);
        self.status.set_zero_flag(shifted_value);
    }

    fn nop(&mut self) {
        self.program_counter.inc();
    }

    fn ora(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        let result = value | self.accumulator.get();
        self.accumulator.set(result);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
    }

    fn pha(&mut self) -> Result<(), StackError> {
        self.stack.push(self.accumulator.get(), &mut self.memory)?;
        Ok(())
    }

    fn php(&mut self) -> Result<(), StackError> {
        let status = self.status.get() | 0b0001_0000;
        self.stack.push(status, &mut self.memory)?;
        Ok(())
    }

    fn pla(&mut self) -> Result<(), StackError> {
        let value = self.stack.pull(&mut self.memory)?;
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        Ok(())
    }

    fn plp(&mut self) -> Result<(), StackError> {
        let value = self.stack.pull(&mut self.memory)?;
        self.status.set(value);
        Ok(())
    }

    fn rol(&mut self, addressing_mode: &AddressingMode) {
        let (old_value, shifted_value) = match addressing_mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = (old_value << 1) + self.status.get_carry_flag();
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let old_value_address = self.get_address(addressing_mode);
                let old_value: u8 = self.memory.read(old_value_address);
                let shifted_value = (old_value << 1) + self.status.get_carry_flag();
                self.memory.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 0b1000_0000 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
    }

    fn ror(&mut self, addressing_mode: &AddressingMode) {
        let (old_value, shifted_value) = match addressing_mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = (old_value >> 1) + (self.status.get_carry_flag() << 7);
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let old_value_address = self.get_address(addressing_mode);
                let old_value: u8 = self.memory.read(old_value_address);
                let shifted_value = (old_value >> 1) + (self.status.get_carry_flag() << 7);
                self.memory.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 1 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
    }

    fn rti(&mut self) -> Result<(), StackError> {
        let status = self.stack.pull(&mut self.memory)?;
        let program_counter_lo = self.stack.pull(&mut self.memory)?;
        let program_counter_hi = self.stack.pull(&mut self.memory)?;
        self.status.set(status);
        self.program_counter
            .set(u16::from_le_bytes([program_counter_lo, program_counter_hi]));
        Ok(())
    }

    fn rts(&mut self) -> Result<(), StackError> {
        let program_counter_lo = self.stack.pull(&mut self.memory)?;
        let program_counter_hi = self.stack.pull(&mut self.memory)?;
        self.program_counter
            .set(u16::from_le_bytes([program_counter_lo, program_counter_hi]).wrapping_add(1));
        Ok(())
    }

    fn sbc(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_value(addressing_mode);
        self.adc_operation(!value);
    }

    fn sec(&mut self) {
        self.status.set_carry_flag_to(true);
    }

    fn sed(&mut self) {
        self.status.set_decimal_mode_flag_to(true);
    }

    fn sei(&mut self) {
        self.status.set_interrupt_disable_flag_to(true);
    }

    fn sta(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address(addressing_mode);
        self.memory.write(address, self.accumulator.get());
    }

    fn stx(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address(addressing_mode);
        self.memory.write(address, self.register_x.get());
    }

    fn sty(&mut self, addressing_mode: &AddressingMode) {
        let address = self.get_address(addressing_mode);
        self.memory.write(address, self.register_y.get());
    }

    fn tax(&mut self) {
        self.register_x.set(self.accumulator.get());
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
    }

    fn tay(&mut self) {
        self.register_y.set(self.accumulator.get());
        self.status.set_zero_flag(self.register_y.get());
        self.status.set_negative_flag(self.register_y.get());
    }

    fn tsx(&mut self) {
        self.register_x.set(self.stack.get_pointer());
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
    }

    fn txa(&mut self) {
        self.accumulator.set(self.register_x.get());
        self.status.set_zero_flag(self.accumulator.get());
        self.status.set_negative_flag(self.accumulator.get());
    }

    fn txs(&mut self) -> Result<(), StackError> {
        let new_pointer = self.register_x.get();
        self.stack.set_pointer(new_pointer)?;
        self.status.set_zero_flag(new_pointer);
        self.status.set_negative_flag(new_pointer);
        Ok(())
    }

    fn tya(&mut self) {
        self.accumulator.set(self.register_y.get());
        self.status.set_zero_flag(self.accumulator.get());
        self.status.set_negative_flag(self.accumulator.get());
    }

    fn next_instruction(&mut self) -> Result<&'static Instruction, UnknownOpCode> {
        let opcode = self.memory.read(self.program_counter.get());
        self.program_counter.inc();
        OPCODES.get(&opcode).ok_or(UnknownOpCode(opcode))
    }

    fn get_address(&mut self, addressing_mode: &AddressingMode) -> u16 {
        let address = match addressing_mode {
            AddressingMode::Absolute => self.memory.read(self.program_counter.get()),
            AddressingMode::AbsoluteX => {
                let value_address: u16 = self.memory.read(self.program_counter.get());
                value_address.wrapping_add(self.register_x.get() as u16)
            }
            AddressingMode::AbsoluteY => {
                let value_address: u16 = self.memory.read(self.program_counter.get());
                value_address.wrapping_add(self.register_y.get() as u16)
            }
            AddressingMode::Immediate | AddressingMode::Relative => self.program_counter.get(),
            AddressingMode::IndexedIndirectX => {
                let indirect_address: u8 = self.memory.read(self.program_counter.get());
                self.memory
                    .read(indirect_address.wrapping_add(self.register_x.get()) as u16)
            }
            AddressingMode::Indirect => {
                let indirect_address = self.memory.read(self.program_counter.get());
                let indirect_address_suffix = indirect_address as u8;

                if (indirect_address_suffix & 0xFF) == 0 {
                    return self.memory.read(indirect_address);
                }

                // Indirect addressing mode is used only in JMP instruction. But an original 6502
                // has does not correctly fetch the target address if the indirect vector falls on
                // a page boundary. This code fixes it.
                // Details: https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
                u16::from_le_bytes([
                    self.memory.read(indirect_address),
                    self.memory.read(u16::from_be_bytes([
                        (indirect_address >> 8) as u8,
                        indirect_address_suffix.wrapping_add(1),
                    ])),
                ])
            }
            AddressingMode::IndirectIndexedY => {
                let indirect_address: u8 = self.memory.read(self.program_counter.get());
                let real_address: u16 = self.memory.read(indirect_address as u16);
                real_address.wrapping_add(self.register_y.get() as u16)
            }
            AddressingMode::ZeroPage => {
                let address: u8 = self.memory.read(self.program_counter.get());
                address as u16
            }
            AddressingMode::ZeroPageX => {
                let address: u8 = self.memory.read(self.program_counter.get());
                address.wrapping_add(self.register_x.get()) as u16
            }
            AddressingMode::ZeroPageY => {
                let address: u8 = self.memory.read(self.program_counter.get());
                address.wrapping_add(self.register_y.get()) as u16
            }
            // TODO: Add error instead of panic
            AddressingMode::Accumulator | AddressingMode::Implied => {
                panic!("Mode {addressing_mode:?} can't have address")
            }
        };
        self.program_counter
            .add(addressing_mode.operand_bytes() as u16);
        address
    }

    fn get_value(&mut self, addressing_mode: &AddressingMode) -> u8 {
        let value_address = self.get_address(addressing_mode);
        self.memory.read(value_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_cpu_with_program(program: Vec<u8>) -> CPU {
        let mut cpu = CPU::new();
        cpu.load_program(program);
        cpu.reset();
        cpu
    }

    #[test]
    fn test_adc_immediate() {
        let mut cpu = setup_cpu_with_program(vec![0x69, 0x01, 0x00]);
        cpu.accumulator.set(0x01);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x02);
        assert!(!cpu.status.is_carry_flag_set());
        assert!(!cpu.status.is_zero_flag_set());
        assert!(!cpu.status.is_overflow_flag_set());
    }

    #[test]
    fn test_asl_accumulator() {
        let mut cpu = setup_cpu_with_program(vec![0x0A, 0x00]);
        cpu.accumulator.set(0b0100_0001);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0b1000_0010);
        assert!(!cpu.status.is_carry_flag_set());
    }

    #[test]
    fn test_bcc_no_carry() {
        let mut cpu = setup_cpu_with_program(vec![0x90, 0x02, 0x00, 0xA9, 0x01, 0x00]);
        cpu.status.set_carry_flag_to(false);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x00);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0x42, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
        assert!(!cpu.status.is_zero_flag_set());
        assert!(!cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_lda_immediate_zero() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0x00, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x00);
        assert!(cpu.status.is_zero_flag_set());
        assert!(!cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_lda_immediate_negative() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0xFF, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0xFF);
        assert!(!cpu.status.is_zero_flag_set());
        assert!(cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_ldy_immediate() {
        let mut cpu = setup_cpu_with_program(vec![0xA0, 0x42, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_y.get(), 0x42);
    }

    #[test]
    fn test_tax() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0x42, 0xAA, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_x.get(), 0x42);
    }

    #[test]
    fn test_tay() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0x42, 0xA8, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_y.get(), 0x42);
    }

    #[test]
    fn test_txa() {
        let mut cpu = setup_cpu_with_program(vec![0xA2, 0x42, 0x8A, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
    }

    #[test]
    fn test_tya() {
        let mut cpu = setup_cpu_with_program(vec![0xA0, 0x42, 0x98, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
    }

    #[test]
    fn test_inc_zero_page() {
        let mut cpu = setup_cpu_with_program(vec![0xE6, 0x42, 0x00]);
        cpu.memory.write(0x42, 0x01u8);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x42);
        assert_eq!(value, 0x02);
        assert!(!cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_dec_zero_page() {
        let mut cpu = setup_cpu_with_program(vec![0xC6, 0x42, 0x00]);
        cpu.memory.write(0x42, 0x01u8);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x42u16);
        assert_eq!(value, 0x00);
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_inx() {
        let mut cpu = setup_cpu_with_program(vec![0xE8, 0x00]);
        cpu.register_x.set(0x01);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_x.get(), 0x02);
        assert!(!cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_dex() {
        let mut cpu = setup_cpu_with_program(vec![0xCA, 0x00]);
        cpu.register_x.set(0x01);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_x.get(), 0x00);
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_iny() {
        let mut cpu = setup_cpu_with_program(vec![0xC8, 0x00]);
        cpu.register_y.set(0x01);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_y.get(), 0x02);
        assert!(!cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_dey() {
        let mut cpu = setup_cpu_with_program(vec![0x88, 0x00]);
        cpu.register_y.set(0x01);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_y.get(), 0x00);
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_jmp_absolute() {
        let mut cpu = setup_cpu_with_program(vec![0x4C, 0x34, 0x12, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.program_counter.get(), 0x1235);
    }

    #[test]
    fn test_jsr_rts() {
        let mut cpu = setup_cpu_with_program(vec![
            0x20, 0x04, 0x80, // JSR $8004
            0x00, // BRK (shouldn't reach here)
            0xA9, 0x42, // LDA #$42
            0x60, // RTS
            0x00, // BRK
        ]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
    }

    #[test]
    fn test_pha_pla() {
        let mut cpu = setup_cpu_with_program(vec![0xA9, 0x42, 0x48, 0xA9, 0x00, 0x68, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
    }

    #[test]
    fn test_php_plp() {
        let mut cpu = setup_cpu_with_program(vec![0x08, 0xA9, 0xFF, 0x28, 0x00]);
        cpu.status.set(0b1100_0000);

        cpu.interpret().unwrap();

        assert!(cpu.status.is_negative_flag_set());
        assert!(cpu.status.is_overflow_flag_set());
    }

    #[test]
    fn test_sec_clc() {
        let mut cpu = setup_cpu_with_program(vec![0x38, 0x18, 0x00]);

        cpu.interpret().unwrap();

        assert!(!cpu.status.is_carry_flag_set());
    }

    #[test]
    fn test_sed_cld() {
        let mut cpu = setup_cpu_with_program(vec![0xF8, 0xD8, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.status.get() & 0b0000_1000, 0);
    }

    #[test]
    fn test_clv() {
        let mut cpu = setup_cpu_with_program(vec![0xB8, 0x00]);
        cpu.status.set_overflow_flag_to(true);

        cpu.interpret().unwrap();

        assert!(!cpu.status.is_overflow_flag_set());
    }

    #[test]
    fn test_bit_zero_page() {
        let mut cpu = setup_cpu_with_program(vec![0x24, 0x42, 0x00]);
        cpu.memory.write(0x42, 0b1100_0000u8);
        cpu.accumulator.set(0b0011_1111);

        cpu.interpret().unwrap();

        assert!(cpu.status.is_negative_flag_set());
        assert!(cpu.status.is_overflow_flag_set());
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_lda_zero_page() {
        let mut cpu = setup_cpu_with_program(vec![0xA5, 0x42, 0x00]);
        cpu.memory.write(0x42, 0xABu8);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0xAB);
        assert!(!cpu.status.is_zero_flag_set());
        assert!(cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_sta_absolute() {
        let mut cpu = setup_cpu_with_program(vec![0x8D, 0x34, 0x12, 0x00]);
        cpu.accumulator.set(0x42);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x1234);
        assert_eq!(value, 0x42);
    }

    #[test]
    fn test_ldx_immediate() {
        let mut cpu = setup_cpu_with_program(vec![0xA2, 0x7F, 0x00]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_x.get(), 0x7F);
        assert!(!cpu.status.is_zero_flag_set());
        assert!(!cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_sty_zero_page_x() {
        let mut cpu = setup_cpu_with_program(vec![0x94, 0x20, 0x00]);
        cpu.register_x.set(0x02);
        cpu.register_y.set(0xAB);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x22);
        assert_eq!(value, 0xAB);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = setup_cpu_with_program(vec![0x69, 0x01, 0x00]);
        cpu.accumulator.set(0xFF);
        cpu.status.set_carry_flag_to(true);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x01);
        assert!(cpu.status.is_carry_flag_set());
        assert!(!cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_sbc_with_borrow() {
        let mut cpu = setup_cpu_with_program(vec![0xE9, 0x01, 0x00]);
        cpu.accumulator.set(0x02);
        cpu.status.set_carry_flag_to(false);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x00);
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_inc_absolute() {
        let mut cpu = setup_cpu_with_program(vec![0xEE, 0x34, 0x12, 0x00]);
        cpu.memory.write(0x1234, 0x7Fu8);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x1234);
        assert_eq!(value, 0x80);
        assert!(cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_and_immediate() {
        let mut cpu = setup_cpu_with_program(vec![0x29, 0b1010_1010, 0x00]);
        cpu.accumulator.set(0b1111_0000);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0b1010_0000);
        assert!(!cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_ora_indirect_y() {
        let mut cpu = setup_cpu_with_program(vec![0x11, 0x20, 0x00]);
        cpu.register_y.set(0x01);
        cpu.memory.write(0x20, 0x34u8);
        cpu.memory.write(0x21, 0x12u8);
        cpu.memory.write(0x1235, 0x0Fu8);
        cpu.accumulator.set(0xF0);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0xFF);
    }

    #[test]
    fn test_eor_zero_page_x() {
        let mut cpu = setup_cpu_with_program(vec![0x55, 0x20, 0x00]);
        cpu.register_x.set(0x02);
        cpu.memory.write(0x22, 0b1010_1010u8);
        cpu.accumulator.set(0b1111_0000);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0b0101_1010);
    }

    #[test]
    fn test_lsr_accumulator() {
        let mut cpu = setup_cpu_with_program(vec![0x4A, 0x00]);
        cpu.accumulator.set(0b0000_0001);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0b0000_0000);
        assert!(cpu.status.is_carry_flag_set());
        assert!(cpu.status.is_zero_flag_set());
    }

    #[test]
    fn test_rol_zero_page() {
        let mut cpu = setup_cpu_with_program(vec![0x26, 0x42, 0x00]);
        cpu.memory.write(0x42, 0b1100_0000u8);
        cpu.status.set_carry_flag_to(true);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x42);
        assert_eq!(value, 0b1000_0001);
        assert!(cpu.status.is_carry_flag_set());
        assert!(cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_bne_taken() {
        let mut cpu = setup_cpu_with_program(vec![0xD0, 0x02, 0xA9, 0x01, 0xA9, 0x02, 0x00]);
        cpu.status.set_zero_flag(0);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x02);
    }

    #[test]
    fn test_beq_not_taken() {
        let mut cpu = setup_cpu_with_program(vec![0xF0, 0x02, 0xA9, 0x01, 0xA9, 0x02, 0x00]);
        cpu.status.set_zero_flag(0);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x02);
    }

    #[test]
    fn test_pha_php_pla_plp() {
        let mut cpu = setup_cpu_with_program(vec![
            0xA9, 0x42, // LDA #$42
            0x48, // PHA
            0xA9, 0x00, // LDA #$00
            0x08, // PHP
            0x28, // PLP
            0x68, // PLA
            0x00, // BRK
        ]);
        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x42);
        assert!(!cpu.status.is_negative_flag_set());
        assert!(!cpu.status.is_overflow_flag_set());
    }

    #[test]
    fn test_clc_sec() {
        let mut cpu = setup_cpu_with_program(vec![0x38, 0x18, 0x00]);

        cpu.interpret().unwrap();

        assert!(!cpu.status.is_carry_flag_set());
    }

    #[test]
    fn test_adc_decimal_mode() {
        let mut cpu = setup_cpu_with_program(vec![0x69, 0x19, 0x00]);
        cpu.accumulator.set(0x23);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x3C);
        assert!(!cpu.status.is_carry_flag_set());
    }

    #[test]
    fn test_sbc_overflow() {
        let mut cpu = setup_cpu_with_program(vec![0xE9, 0x7F, 0x00]);
        cpu.accumulator.set(0x80);
        cpu.status.set_carry_flag_to(true);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x01);
        assert!(cpu.status.is_carry_flag_set());
        assert!(cpu.status.is_overflow_flag_set());
    }

    #[test]
    fn test_bit_absolute() {
        let mut cpu = setup_cpu_with_program(vec![0x2C, 0x34, 0x12, 0x00]);
        cpu.memory.write(0x1234, 0b0100_0000u8);
        cpu.accumulator.set(0b0000_0000);

        cpu.interpret().unwrap();

        assert!(cpu.status.is_zero_flag_set());
        assert!(cpu.status.is_overflow_flag_set());
        assert!(!cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_asl_zero_page_x() {
        let mut cpu = setup_cpu_with_program(vec![0x16, 0x20, 0x00]);
        cpu.register_x.set(0x02);
        cpu.memory.write(0x22, 0b0100_0001u8);

        cpu.interpret().unwrap();

        let value: u8 = cpu.memory.read(0x22);
        assert_eq!(value, 0b1000_0010);
        assert!(!cpu.status.is_carry_flag_set());
    }

    #[test]
    fn test_ror_accumulator_with_carry() {
        let mut cpu = setup_cpu_with_program(vec![0x6A, 0x00]);
        cpu.accumulator.set(0b0000_0001);
        cpu.status.set_carry_flag_to(true);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0b1000_0000);
        assert!(cpu.status.is_carry_flag_set());
        assert!(cpu.status.is_negative_flag_set());
    }

    #[test]
    fn test_bvc_no_overflow() {
        let mut cpu = setup_cpu_with_program(vec![0x50, 0x02, 0xA9, 0x01, 0xA9, 0x02, 0x00]);
        cpu.status.set_overflow_flag_to(false);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x02);
    }

    #[test]
    fn test_bvs_overflow() {
        let mut cpu = setup_cpu_with_program(vec![0x70, 0x02, 0xA9, 0x01, 0xA9, 0x02, 0x00]);
        cpu.status.set_overflow_flag_to(true);

        cpu.interpret().unwrap();

        assert_eq!(cpu.accumulator.get(), 0x02);
    }

    #[test]
    fn test_tsx_txs() {
        let mut cpu = setup_cpu_with_program(vec![0xBA, 0x9A, 0x00]);
        cpu.stack.set_pointer(0xAB).unwrap();

        cpu.interpret().unwrap();

        assert_eq!(cpu.register_x.get(), 0xAB);
        assert_eq!(cpu.stack.get_pointer(), 0xAB);
    }

    #[test]
    fn test_rts_returns_correctly() {
        let mut cpu = setup_cpu_with_program(vec![
            0x20, 0x04, 0x80, // JSR $8004
            0x00, // BRK
            0x60, // RTS
        ]);

        cpu.interpret().unwrap();

        assert_eq!(cpu.program_counter.get(), 0x8004);
    }

    #[test]
    fn test_jmp_indirect_page_boundary() {
        let mut cpu = setup_cpu_with_program(vec![0x6C, 0xFF, 0x00, 0x00]);
        cpu.memory.write(0x00FF, 0x34u8);
        cpu.memory.write(0x0000, 0x12u8);

        cpu.interpret().unwrap();

        assert_eq!(cpu.program_counter.get(), 0x1235);
    }
}
