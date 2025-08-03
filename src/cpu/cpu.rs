use crate::cpu::bus::{CPUBus, IOOperation};
use crate::cpu::error::{StackError, UnknownOpCode};
use crate::cpu::opcode::OPCODES;
use crate::cpu::opcode::{AddressingMode, Instruction, OpCode};
use crate::cpu::register::counter::ProgramCounter;
use crate::cpu::register::register::Register;
use crate::cpu::register::stack::Stack;
use crate::cpu::register::status::Status;
use std::error::Error;

type PageCrossed = bool;

pub struct CPU {
    accumulator: Register<u8>,
    register_x: Register<u8>,
    register_y: Register<u8>,
    program_counter: ProgramCounter,
    status: Status,
    pub bus: CPUBus,
    stack: Stack,
}

impl CPU {
    pub fn new(bus: CPUBus) -> Self {
        CPU {
            accumulator: Register::new(0),
            register_x: Register::new(0),
            register_y: Register::new(0),
            program_counter: ProgramCounter::new(),
            status: Status::new(),
            stack: Stack::new(),
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.program_counter.set(self.bus.read(0xFFFC));
        self.accumulator.set(0);
        self.register_x.set(0);
        self.register_y.set(0);
        self.status.reset();
        self.stack.reset();
    }

    pub fn run<F>(&mut self, mut callback: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&mut CPU),
    {
        loop {
            callback(self);
            let instruction = self.next_instruction()?;
            let passed_cycles = match instruction.opcode {
                OpCode::ADC => self.adc(&instruction),
                OpCode::AND => self.and(&instruction),
                OpCode::ASL => self.asl(&instruction),
                OpCode::BCC => self.bcc(&instruction),
                OpCode::BCS => self.bcs(&instruction),
                OpCode::BEQ => self.beq(&instruction),
                OpCode::BIT => self.bit(&instruction),
                OpCode::BMI => self.bmi(&instruction),
                OpCode::BNE => self.bne(&instruction),
                OpCode::BPL => self.bpl(&instruction),
                OpCode::BRK => return Ok(()),
                OpCode::BVC => self.bvc(&instruction),
                OpCode::BVS => self.bvs(&instruction),
                OpCode::CLC => self.clc(&instruction),
                OpCode::CLD => self.cld(&instruction),
                OpCode::CLI => self.cli(&instruction),
                OpCode::CLV => self.clv(&instruction),
                OpCode::CMP => self.cmp(&instruction),
                OpCode::CPX => self.cpx(&instruction),
                OpCode::CPY => self.cpy(&instruction),
                OpCode::DEC => self.dec(&instruction),
                OpCode::DEX => self.dex(&instruction),
                OpCode::DEY => self.dey(&instruction),
                OpCode::EOR => self.eor(&instruction),
                OpCode::INC => self.inc(&instruction),
                OpCode::INX => self.inx(&instruction),
                OpCode::INY => self.iny(&instruction),
                OpCode::JMP => self.jmp(&instruction),
                OpCode::JSR => self.jsr(&instruction)?,
                OpCode::LDA => self.lda(&instruction),
                OpCode::LDX => self.ldx(&instruction),
                OpCode::LDY => self.ldy(&instruction),
                OpCode::LSR => self.lsr(&instruction),
                OpCode::NOP => self.nop(&instruction),
                OpCode::ORA => self.ora(&instruction),
                OpCode::PHA => self.pha(&instruction)?,
                OpCode::PHP => self.php(&instruction)?,
                OpCode::PLA => self.pla(&instruction)?,
                OpCode::PLP => self.plp(&instruction)?,
                OpCode::ROL => self.rol(&instruction),
                OpCode::ROR => self.ror(&instruction),
                OpCode::RTI => self.rti(&instruction)?,
                OpCode::RTS => self.rts(&instruction)?,
                OpCode::SBC => self.sbc(&instruction),
                OpCode::SEC => self.sec(&instruction),
                OpCode::SED => self.sed(&instruction),
                OpCode::SEI => self.sei(&instruction),
                OpCode::STA => self.sta(&instruction),
                OpCode::STX => self.stx(&instruction),
                OpCode::STY => self.sty(&instruction),
                OpCode::TAX => self.tax(&instruction),
                OpCode::TAY => self.tay(&instruction),
                OpCode::TSX => self.tsx(&instruction),
                OpCode::TXA => self.txa(&instruction),
                OpCode::TXS => self.txs(&instruction)?,
                OpCode::TYA => self.tya(&instruction),
                OpCode::AAC => self.aac(&instruction),
                OpCode::SAX => self.sax(&instruction),
                OpCode::ARR => self.arr(&instruction),
                OpCode::ASR => self.asr(&instruction),
                OpCode::ATX => self.atx(&instruction),
                OpCode::AXA => self.axa(&instruction),
                OpCode::AXS => self.axs(&instruction),
                OpCode::DCP => self.dcp(&instruction),
                OpCode::DOP => self.dop(&instruction),
                OpCode::ISB => self.isb(&instruction),
                OpCode::KIL => return Ok(()),
                OpCode::LAR => self.lar(&instruction)?,
                OpCode::LAX => self.lax(&instruction),
                OpCode::RLA => self.rla(&instruction),
                OpCode::RRA => self.rra(&instruction),
                OpCode::SLO => self.slo(&instruction),
                OpCode::SRE => self.sre(&instruction),
                OpCode::SXA => self.sxa(&instruction),
                OpCode::SYA => self.sya(&instruction),
                OpCode::TOP => self.top(&instruction),
                OpCode::XAA => panic!("XAA encountered. Exact behaviour is unknown."),
                OpCode::XAS => self.xas(&instruction)?,
            };
            self.bus.tick(passed_cycles);
        }
    }

    fn adc(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.adc_operation(value);
        instruction.cycles + page_crossed as u8
    }

    // Moved ADC instruction's logic to separate function, because the same logic
    // is reused in the SBC instruction.
    fn adc_operation(&mut self, value: u8) {
        let (result, no_borrow_add_carry) = self.accumulator.add(value);
        let (result, borrow_add_carry) =
            result.overflowing_add(self.status.is_carry_flag_set() as u8);
        self.status
            .set_carry_flag_to(borrow_add_carry | no_borrow_add_carry);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        self.status
            .set_overflow_flag_to((value ^ result) & (result ^ self.accumulator.get()) & 0x80 != 0);
        self.accumulator.set(result);
    }

    fn and(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, mut value) = self.get_value(&instruction.mode);
        value &= self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn asl(&mut self, instruction: &Instruction) -> u8 {
        let addressing_mode = &instruction.mode;
        let (old_value, shifted_value) = match addressing_mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = old_value << 1;
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let (_, old_value_address) = self.read_operand_address(addressing_mode);
                let old_value: u8 = self.bus.read(old_value_address);
                let shifted_value = old_value << 1;
                self.bus.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 0b1000_0000 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
        instruction.cycles
    }

    fn bcc(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if !self.status.is_carry_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bcs(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if self.status.is_carry_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn beq(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if self.status.is_zero_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bit(&mut self, instruction: &Instruction) -> u8 {
        let (_, value) = self.get_value(&instruction.mode);
        self.status.set_zero_flag(value & self.accumulator.get());
        self.status.set_negative_flag(value);
        self.status.set_overflow_flag_to(value & 0b0100_0000 != 0);
        instruction.cycles
    }

    fn bmi(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if self.status.is_negative_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bne(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if !self.status.is_zero_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bpl(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if !self.status.is_negative_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bvc(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if !self.status.is_overflow_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn bvs(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, offset) = self.get_value(&instruction.mode);
        if self.status.is_overflow_flag_set() {
            self.program_counter.move_with_offset(offset);
            instruction.cycles + if page_crossed { 2 } else { 1 }
        } else {
            instruction.cycles
        }
    }

    fn clc(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_carry_flag_to(false);
        instruction.cycles
    }

    fn cld(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_decimal_mode_flag_to(false);
        instruction.cycles
    }

    fn cli(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_interrupt_disable_flag_to(false);
        instruction.cycles
    }

    fn clv(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_overflow_flag_to(false);
        instruction.cycles
    }

    fn cmp(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        let result = self.accumulator.sub(value);
        self.status
            .set_carry_flag_to(self.accumulator.get() >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        instruction.cycles + page_crossed as u8
    }

    fn cpx(&mut self, instruction: &Instruction) -> u8 {
        let (_, value) = self.get_value(&instruction.mode);
        let result = self.register_x.sub(value);
        self.status
            .set_carry_flag_to(self.register_x.get() >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        instruction.cycles
    }

    fn cpy(&mut self, instruction: &Instruction) -> u8 {
        let (_, value) = self.get_value(&instruction.mode);
        let result = self.register_y.sub(value);
        self.status
            .set_carry_flag_to(self.register_y.get() >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);
        instruction.cycles
    }

    fn dec(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let value = IOOperation::<u8>::read(&mut self.bus, address).wrapping_sub(1);
        self.bus.write(address, value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn dex(&mut self, instruction: &Instruction) -> u8 {
        self.register_x.dec();
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
        instruction.cycles
    }

    fn dey(&mut self, instruction: &Instruction) -> u8 {
        self.register_y.dec();
        self.status.set_zero_flag(self.register_y.get());
        self.status.set_negative_flag(self.register_y.get());
        instruction.cycles
    }

    fn eor(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, mut value) = self.get_value(&instruction.mode);
        value ^= self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn inc(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let value = IOOperation::<u8>::read(&mut self.bus, address).wrapping_add(1);
        self.bus.write(address, value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn inx(&mut self, instruction: &Instruction) -> u8 {
        let value = self.register_x.inc();
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn iny(&mut self, instruction: &Instruction) -> u8 {
        let value = self.register_y.inc();
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn jmp(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.program_counter.set(address);
        instruction.cycles
    }

    fn jsr(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let current_address_bytes: [u8; 2] =
            self.program_counter.get().wrapping_sub(1).to_be_bytes();
        self.stack.push(current_address_bytes[0], &mut self.bus)?;
        self.stack.push(current_address_bytes[1], &mut self.bus)?;
        self.program_counter.set(address);
        Ok(instruction.cycles)
    }

    fn lda(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn ldx(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.register_x.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn ldy(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.register_y.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn lsr(&mut self, instruction: &Instruction) -> u8 {
        let (old_value, shifted_value) = match instruction.mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = old_value >> 1;
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let (_, old_value_address) = self.read_operand_address(&instruction.mode);
                let old_value: u8 = self.bus.read(old_value_address);
                let shifted_value = old_value >> 1;
                self.bus.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 1 != 0);
        self.status.set_negative_flag(0);
        self.status.set_zero_flag(shifted_value);
        instruction.cycles
    }

    fn nop(&mut self, instruction: &Instruction) -> u8 {
        instruction.cycles
    }

    fn ora(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, mut value) = self.get_value(&instruction.mode);
        value |= self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn pha(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        self.stack.push(self.accumulator.get(), &mut self.bus)?;
        Ok(instruction.cycles)
    }

    fn php(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let status = self.status.get() | 0b0001_0000;
        self.stack.push(status, &mut self.bus)?;
        Ok(instruction.cycles)
    }

    fn pla(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let value = self.stack.pull(&mut self.bus)?;
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        Ok(instruction.cycles)
    }

    fn plp(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let value = self.stack.pull(&mut self.bus)? & 0xEF | 0x20;
        self.status.set(value);
        Ok(instruction.cycles)
    }

    fn rol(&mut self, instruction: &Instruction) -> u8 {
        let (old_value, shifted_value) = match instruction.mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value = (old_value << 1).wrapping_add(self.status.get_carry_flag());
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let (_, old_value_address) = self.read_operand_address(&instruction.mode);
                let old_value: u8 = self.bus.read(old_value_address);
                let shifted_value = (old_value << 1).wrapping_add(self.status.get_carry_flag());
                self.bus.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 0b1000_0000 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
        instruction.cycles
    }

    fn ror(&mut self, instruction: &Instruction) -> u8 {
        let (old_value, shifted_value) = match instruction.mode {
            AddressingMode::Accumulator => {
                let old_value = self.accumulator.get();
                let shifted_value =
                    (old_value >> 1).wrapping_add(self.status.get_carry_flag() << 7);
                self.accumulator.set(shifted_value);
                (old_value, shifted_value)
            }
            _ => {
                let (_, old_value_address) = self.read_operand_address(&instruction.mode);
                let old_value: u8 = self.bus.read(old_value_address);
                let shifted_value =
                    (old_value >> 1).wrapping_add(self.status.get_carry_flag() << 7);
                self.bus.write(old_value_address, shifted_value);
                (old_value, shifted_value)
            }
        };
        self.status.set_carry_flag_to(old_value & 1 != 0);
        self.status.set_negative_flag(shifted_value);
        self.status.set_zero_flag(shifted_value);
        instruction.cycles
    }

    fn rti(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let status = self.stack.pull(&mut self.bus)?;
        let program_counter_lo = self.stack.pull(&mut self.bus)?;
        let program_counter_hi = self.stack.pull(&mut self.bus)?;
        self.status.set(status);
        self.program_counter
            .set(u16::from_le_bytes([program_counter_lo, program_counter_hi]));
        Ok(instruction.cycles)
    }

    fn rts(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let program_counter_lo = self.stack.pull(&mut self.bus)?;
        let program_counter_hi = self.stack.pull(&mut self.bus)?;
        self.program_counter
            .set(u16::from_le_bytes([program_counter_lo, program_counter_hi]).wrapping_add(1));
        Ok(instruction.cycles)
    }

    fn sbc(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.adc_operation(!value);
        instruction.cycles + page_crossed as u8
    }

    fn sec(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_carry_flag_to(true);
        instruction.cycles
    }

    fn sed(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_decimal_mode_flag_to(true);
        instruction.cycles
    }

    fn sei(&mut self, instruction: &Instruction) -> u8 {
        self.status.set_interrupt_disable_flag_to(true);
        instruction.cycles
    }

    fn sta(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.bus.write(address, self.accumulator.get());
        instruction.cycles
    }

    fn stx(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.bus.write(address, self.register_x.get());
        instruction.cycles
    }

    fn sty(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.bus.write(address, self.register_y.get());
        instruction.cycles
    }

    fn tax(&mut self, instruction: &Instruction) -> u8 {
        self.register_x.set(self.accumulator.get());
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
        instruction.cycles
    }

    fn tay(&mut self, instruction: &Instruction) -> u8 {
        self.register_y.set(self.accumulator.get());
        self.status.set_zero_flag(self.register_y.get());
        self.status.set_negative_flag(self.register_y.get());
        instruction.cycles
    }

    fn tsx(&mut self, instruction: &Instruction) -> u8 {
        self.register_x.set(self.stack.get_pointer());
        self.status.set_zero_flag(self.register_x.get());
        self.status.set_negative_flag(self.register_x.get());
        instruction.cycles
    }

    fn txa(&mut self, instruction: &Instruction) -> u8 {
        self.accumulator.set(self.register_x.get());
        self.status.set_zero_flag(self.accumulator.get());
        self.status.set_negative_flag(self.accumulator.get());
        instruction.cycles
    }

    fn txs(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let new_pointer = self.register_x.get();
        self.stack.set_pointer(new_pointer)?;
        Ok(instruction.cycles)
    }

    fn tya(&mut self, instruction: &Instruction) -> u8 {
        self.accumulator.set(self.register_y.get());
        self.status.set_zero_flag(self.accumulator.get());
        self.status.set_negative_flag(self.accumulator.get());
        instruction.cycles
    }

    fn aac(&mut self, instruction: &Instruction) -> u8 {
        let (_, mut value) = self.get_value(&instruction.mode);
        value &= self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        self.status.set_carry_flag_to(value & 0b1000_0000 != 0);
        instruction.cycles
    }

    fn sax(&mut self, instruction: &Instruction) -> u8 {
        let value = self.register_x.get() & self.accumulator.get();
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.bus.write(address, value);
        instruction.cycles
    }

    fn arr(&mut self, instruction: &Instruction) -> u8 {
        let (_, mut value) = self.get_value(&instruction.mode);
        value = (value & self.accumulator.get()) >> 1;
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);

        match (value >> 5) & 0x02 {
            0b11 => {
                self.status.set_carry_flag_to(true);
                self.status.set_overflow_flag_to(false);
            }
            0b10 => {
                self.status.set_carry_flag_to(true);
                self.status.set_overflow_flag_to(true);
            }
            0b01 => {
                self.status.set_carry_flag_to(false);
                self.status.set_overflow_flag_to(true);
            }
            _ => {
                self.status.set_carry_flag_to(false);
                self.status.set_overflow_flag_to(false);
            }
        };

        instruction.cycles
    }

    fn asr(&mut self, instruction: &Instruction) -> u8 {
        let (_, mut value) = self.get_value(&instruction.mode);
        value &= self.accumulator.get();
        let shifted_value = value >> 1;
        self.accumulator.set(shifted_value);
        self.status.set_zero_flag(shifted_value);
        self.status.set_negative_flag(shifted_value);
        self.status.set_carry_flag_to(value & 0b0000_0001 != 0);
        instruction.cycles
    }

    fn atx(&mut self, instruction: &Instruction) -> u8 {
        let (_, mut value) = self.get_value(&instruction.mode);
        value &= self.accumulator.get();
        self.register_x.set(value);
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn axa(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        self.bus.write(
            address,
            self.register_x.get() & self.accumulator.get() & address.to_be_bytes()[0],
        );
        instruction.cycles
    }

    fn axs(&mut self, instruction: &Instruction) -> u8 {
        let (_, mut value) = self.get_value(&instruction.mode);
        value = (self.accumulator.get() & self.register_x.get()).wrapping_sub(value);
        self.register_x.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn dcp(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let value = IOOperation::<u8>::read(&mut self.bus, address).wrapping_sub(1);
        self.bus.write(address, value);

        let result = self.accumulator.sub(value);
        self.status
            .set_carry_flag_to(self.accumulator.get() >= value);
        self.status.set_zero_flag(result);
        self.status.set_negative_flag(result);

        instruction.cycles
    }

    fn dop(&mut self, instruction: &Instruction) -> u8 {
        self.program_counter.inc();
        instruction.cycles
    }

    fn isb(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let value = IOOperation::<u8>::read(&mut self.bus, address).wrapping_add(1);
        self.bus.write(address, value);
        self.adc_operation(!value);
        instruction.cycles
    }

    fn lar(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let (page_crossed, mut value) = self.get_value(&instruction.mode);
        value &= self.stack.get_pointer();
        self.register_x.set(value);
        self.accumulator.set(value);
        self.stack.set_pointer(value)?;
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        Ok(instruction.cycles + page_crossed as u8)
    }

    fn lax(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, value) = self.get_value(&instruction.mode);
        self.register_x.set(value);
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles + page_crossed as u8
    }

    fn rla(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let mut value: u8 = self.bus.read(address);
        let carry_flag = self.status.get_carry_flag();

        self.status.set_carry_flag_to(value & 0b1000_0000 != 0);
        value = (value << 1).wrapping_add(carry_flag);
        self.bus.write(address, value);

        value = value & self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn rra(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let mut value: u8 = self.bus.read(address);
        let set_carry_flag = value & 0b0000_0001 != 0;
        value = (value >> 1).wrapping_add(self.status.get_carry_flag() << 7);
        self.status.set_carry_flag_to(set_carry_flag);
        self.bus.write(address, value);
        self.adc_operation(value);
        instruction.cycles
    }

    fn slo(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let mut value: u8 = self.bus.read(address);

        self.status.set_carry_flag_to(value & 0b1000_0000 != 0);
        value <<= 1;
        self.bus.write(address, value);

        value = value | self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn sre(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let mut value: u8 = self.bus.read(address);

        self.status.set_carry_flag_to(value & 0b0000_0001 != 0);
        value >>= 1;
        self.bus.write(address, value);

        value = value ^ self.accumulator.get();
        self.accumulator.set(value);
        self.status.set_zero_flag(value);
        self.status.set_negative_flag(value);
        instruction.cycles
    }

    fn sxa(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let result = (self.register_x.get() & address.to_be_bytes()[0]).wrapping_add(1);
        self.bus.write(address, result);
        instruction.cycles
    }

    fn sya(&mut self, instruction: &Instruction) -> u8 {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let result = (self.register_y.get() & address.to_be_bytes()[0]).wrapping_add(1);
        self.bus.write(address, result);
        instruction.cycles
    }

    fn top(&mut self, instruction: &Instruction) -> u8 {
        let (page_crossed, _) = self.read_operand_address(&instruction.mode);
        instruction.cycles + page_crossed as u8
    }

    fn xas(&mut self, instruction: &Instruction) -> Result<u8, StackError> {
        let (_, address) = self.read_operand_address(&instruction.mode);
        let result = self.register_x.get() & self.accumulator.get();
        self.stack.set_pointer(result)?;
        self.bus
            .write(address, (result & address.to_be_bytes()[0]).wrapping_add(1));
        Ok(instruction.cycles)
    }

    fn next_instruction(&mut self) -> Result<&'static Instruction, UnknownOpCode> {
        let opcode = self.bus.read(self.program_counter.get());
        self.program_counter.inc();
        OPCODES.get(&opcode).ok_or(UnknownOpCode(opcode))
    }

    fn read_operand_address(&mut self, addressing_mode: &AddressingMode) -> (PageCrossed, u16) {
        let result = self.get_operand_address(addressing_mode, self.program_counter.get());
        self.program_counter
            .add(addressing_mode.operand_bytes() as u16);
        result
    }

    fn get_operand_address(
        &mut self,
        addressing_mode: &AddressingMode,
        address: u16,
    ) -> (PageCrossed, u16) {
        match addressing_mode {
            AddressingMode::Absolute => (false, self.bus.read(address)),
            AddressingMode::AbsoluteX => {
                let absolute_address: u16 = self.bus.read(address);
                let absolute_address_x =
                    absolute_address.wrapping_add(self.register_x.get() as u16);
                (
                    (absolute_address >> 8) != (absolute_address_x >> 8),
                    absolute_address_x,
                )
            }
            AddressingMode::AbsoluteY => {
                let absolute_address: u16 = self.bus.read(address);
                let absolute_address_y =
                    absolute_address.wrapping_add(self.register_y.get() as u16);
                (
                    (absolute_address >> 8) != (absolute_address_y >> 8),
                    absolute_address_y,
                )
            }
            AddressingMode::Immediate | AddressingMode::Relative => (false, address),
            AddressingMode::IndexedIndirectX => {
                let indirect_address: u8 = IOOperation::<u8>::read(&mut self.bus, address)
                    .wrapping_add(self.register_x.get());
                // TODO: Maybe it is better to move this logic into the bus
                if (indirect_address & 0xFF) == 0 {
                    (false, self.bus.read(indirect_address as u16))
                } else {
                    (
                        false,
                        u16::from_le_bytes([
                            self.bus.read(indirect_address as u16),
                            self.bus.read(indirect_address.wrapping_add(1) as u16),
                        ]),
                    )
                }
            }
            AddressingMode::Indirect => {
                let indirect_address = self.bus.read(address);
                let indirect_address_suffix = indirect_address as u8;

                // TODO: Maybe it is better to move this logic into the bus
                // Indirect addressing mode is used only in JMP instruction. But an original 6502
                // has does not correctly fetch the target address if the indirect vector falls on
                // a page boundary. This code fixes it.
                // Details: https://www.nesdev.org/obelisk-6502-guide/reference.html#JMP
                if (indirect_address_suffix & 0xFF) == 0 {
                    (false, self.bus.read(indirect_address))
                } else {
                    (
                        false,
                        u16::from_le_bytes([
                            self.bus.read(indirect_address),
                            self.bus.read(u16::from_be_bytes([
                                (indirect_address >> 8) as u8,
                                indirect_address_suffix.wrapping_add(1),
                            ])),
                        ]),
                    )
                }
            }
            AddressingMode::IndirectIndexedY => {
                let indirect_address: u8 = self.bus.read(address);
                // TODO: Maybe it is better to move this logic into the bus
                let real_address = if (indirect_address & 0xFF) == 0 {
                    self.bus.read(indirect_address as u16)
                } else {
                    u16::from_le_bytes([
                        self.bus.read(indirect_address as u16),
                        self.bus.read(indirect_address.wrapping_add(1) as u16),
                    ])
                };
                (
                    false,
                    real_address.wrapping_add(self.register_y.get() as u16),
                )
            }
            AddressingMode::ZeroPage => (
                false,
                IOOperation::<u8>::read(&mut self.bus, address) as u16,
            ),
            AddressingMode::ZeroPageX => (
                false,
                IOOperation::<u8>::read(&mut self.bus, address).wrapping_add(self.register_x.get())
                    as u16,
            ),
            AddressingMode::ZeroPageY => (
                false,
                IOOperation::<u8>::read(&mut self.bus, address).wrapping_add(self.register_y.get())
                    as u16,
            ),
            // TODO: Add error instead of panic
            AddressingMode::Accumulator | AddressingMode::Implied => {
                panic!("Mode {addressing_mode:?} can't have address")
            }
        }
    }

    fn get_value(&mut self, addressing_mode: &AddressingMode) -> (PageCrossed, u8) {
        let (page_crossed, address) = self.read_operand_address(addressing_mode);
        (page_crossed, self.bus.read(address))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rom::rom::Rom;
    use std::fs::read_to_string;

    // Start execution at $C000 and compare execution with a known
    // good log - https://www.qmtpro.com/~nes/misc/nestest.log
    #[test]
    fn test_nestest_cpu_instructions() {
        let logs_file = read_to_string("./ines/tests/nestest.log").unwrap();
        let mut logs = logs_file.lines().map(|l| l.to_string());
        let rom_content = std::fs::read("./ines/tests/nestest.nes").unwrap();
        let mut cpu = setup_cpu_with_program(rom_content);
        cpu.program_counter.set(0xC000);

        cpu.run(|cpu| {
            let trace_log = trace(cpu);
            println!("{trace_log}");
            assert_eq!(trace_log, logs.next().unwrap());
        })
        .unwrap();
    }

    fn setup_cpu_with_program(program: Vec<u8>) -> CPU {
        let rom = Rom::new(&program).unwrap();
        let bus = CPUBus::new(rom);
        let mut cpu = CPU::new(bus);
        cpu.reset();
        cpu
    }

    fn trace(cpu: &mut CPU) -> String {
        let program_counter = cpu.program_counter.get();
        let raw_opcode = cpu.bus.read(program_counter);
        let opcode = OPCODES
            .get(&raw_opcode)
            .ok_or(UnknownOpCode(raw_opcode))
            .unwrap();

        let mut hex_dump = vec![raw_opcode];

        let (mem_addr, stored_value) = match opcode.mode {
            AddressingMode::Immediate
            | AddressingMode::Accumulator
            | AddressingMode::Implied
            | AddressingMode::Relative => (0, 0),
            _ => {
                let (_, addr) = cpu.get_operand_address(&opcode.mode, program_counter + 1);
                let addr_value: u8 = cpu.bus.read(addr);
                (addr, addr_value)
            }
        };

        let tmp = match opcode.mode.operand_bytes() {
            0 => match opcode.mode {
                AddressingMode::Accumulator => "A ".to_string(),
                _ => "".to_string(),
            },
            1 => {
                let address: u8 = cpu.bus.read(program_counter + 1);
                hex_dump.push(address);

                match opcode.mode {
                    AddressingMode::Immediate => format!("#${:02x}", address),
                    AddressingMode::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                    AddressingMode::ZeroPageX => format!(
                        "${:02x},X @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::ZeroPageY => format!(
                        "${:02x},Y @ {:02x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    AddressingMode::IndexedIndirectX => format!(
                        "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                        address,
                        address.wrapping_add(cpu.register_x.get()),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::IndirectIndexedY => format!(
                        "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                        address,
                        mem_addr.wrapping_sub(cpu.register_y.get() as u16),
                        mem_addr,
                        stored_value
                    ),
                    AddressingMode::Accumulator
                    | AddressingMode::Implied
                    | AddressingMode::Relative => {
                        let address: usize =
                            (program_counter as usize + 2).wrapping_add((address as i8) as usize);
                        format!("${:04x}", address)
                    }
                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 2. code {}",
                        opcode.mode, opcode.opcode
                    ),
                }
            }
            2 => {
                let address_lo = cpu.bus.read(program_counter + 1);
                let address_hi = cpu.bus.read(program_counter + 2);
                hex_dump.push(address_lo);
                hex_dump.push(address_hi);

                let address = cpu.bus.read(program_counter + 1);

                match (&opcode.opcode, &opcode.mode) {
                    (_, AddressingMode::Indirect) => {
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo: u8 = cpu.bus.read(address);
                            let hi: u8 = cpu.bus.read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.bus.read(address)
                        };
                        format!("(${:04x}) = {mem_addr:04x}", address)
                    }
                    (
                        _,
                        AddressingMode::Accumulator
                        | AddressingMode::Implied
                        | AddressingMode::Relative,
                    ) => {
                        format!("${:04x}", address)
                    }
                    (OpCode::JMP | OpCode::JSR, AddressingMode::Absolute) => {
                        format!("${:04x}", mem_addr)
                    }
                    (_, AddressingMode::Absolute) => {
                        format!("${:04x} = {:02x}", mem_addr, stored_value)
                    }
                    (_, AddressingMode::AbsoluteX) => format!(
                        "${:04x},X @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    (_, AddressingMode::AbsoluteY) => format!(
                        "${:04x},Y @ {:04x} = {:02x}",
                        address, mem_addr, stored_value
                    ),
                    _ => panic!(
                        "unexpected addressing mode {:?} has ops-len 3. code {}",
                        opcode.mode, opcode.opcode
                    ),
                }
            }
            _ => String::from(""),
        };

        let hex_str = hex_dump
            .iter()
            .map(|z| format!("{:02x}", z))
            .collect::<Vec<String>>()
            .join(" ");
        let asm_str = format!(
            "{:04x}  {:8} {: >4} {}",
            program_counter,
            hex_str,
            match raw_opcode {
                0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => "*NOP".to_string(),
                0xEB => "*SBC".to_string(),
                _ => opcode.opcode.to_string(),
            },
            tmp
        )
        .trim()
        .to_string();

        format!(
            "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
            asm_str,
            cpu.accumulator.get(),
            cpu.register_x.get(),
            cpu.register_y.get(),
            cpu.status.get(),
            cpu.stack.get_pointer(),
        )
        .to_ascii_uppercase()
    }
}
