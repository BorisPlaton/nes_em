use crate::cpu::opcode::OPCODES;
use crate::cpu::error::{StackError, UnknownOpCode};
use crate::cpu::opcode::{AddressingMode, Instruction, OpCode};
use crate::cpu::register::counter::ProgramCounter;
use crate::cpu::register::register::Register;
use crate::cpu::register::status::Status;
use crate::cpu::register::stack::Stack;
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
        let signed_value = value as i8;
        let signed_accumulator_value = self.accumulator.get() as i8;
        let carry_bit = self.status.is_carry_flag_set() as u8;
        let (result, is_carry_flag_set) = self.accumulator.add(value + carry_bit);
        let signed_result = signed_value
            .wrapping_add(signed_accumulator_value)
            .wrapping_add(carry_bit as i8);
        #[rustfmt::skip]
        let is_overflow_flag_set = (
            signed_value >= 0 &&
            signed_accumulator_value >= 0
            && carry_bit == 1
            && signed_result < 0
        ) || (
            signed_value < 0 &&
            signed_accumulator_value < 0 &&
            carry_bit == 0 &&
            signed_result > 0
        );

        self.status.set_carry_flag_to(is_carry_flag_set);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        self.status.set_overflow_flag_to(is_overflow_flag_set);
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
        self.status.set_carry_flag_to(old_value & 0x10 != 0);
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
        self.status.set_carry_flag_to(old_value & 0x10 != 0);
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
        self.status.set_negative_flag(0);
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
            .set(u16::from_le_bytes([program_counter_lo, program_counter_hi]) + 1);
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
                let hi_indirect_address_suffix: u8 = indirect_address_suffix.wrapping_add(1);
                let address_prefix = (indirect_address >> 8) as u8;
                u16::from_le_bytes([
                    self.memory.read(indirect_address),
                    self.memory.read(u16::from_be_bytes([
                        address_prefix,
                        hi_indirect_address_suffix,
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
