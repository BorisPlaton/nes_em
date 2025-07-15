use crate::cpu::codes::OPCODES;
use crate::cpu::error::{StackError, UnknownOpCode};
use crate::cpu::opcode::{AddressingMode, Instruction, OpCode};
use crate::cpu::register::register::Register;
use crate::cpu::register::status::Status;
use crate::cpu::stack::Stack;
use crate::mem::map::{IOOperation, MemoryMap};
use std::error::Error;

pub struct CPU {
    accumulator: Register<u8>,
    register_x: Register<u8>,
    register_y: Register<u8>,
    program_counter: Register<u16>,
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
            program_counter: Register::new(0),
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
                OpCode::JSR => self.jsr(&instruction.mode),
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
                OpCode::RTI => self.rti(&instruction.mode),
                OpCode::RTS => self.rts(&instruction.mode),
                OpCode::SBC => self.sbc(&instruction.mode),
                OpCode::SEC => self.sec(&instruction.mode),
                OpCode::SED => self.sed(&instruction.mode),
                OpCode::SEI => self.sei(&instruction.mode),
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

    fn adc(&mut self, addressing_mode: &AddressingMode) {}

    fn and(&mut self, addressing_mode: &AddressingMode) {}

    fn asl(&mut self, addressing_mode: &AddressingMode) {}

    fn bcc(&mut self, addressing_mode: &AddressingMode) {}

    fn bcs(&mut self, addressing_mode: &AddressingMode) {}

    fn beq(&mut self, addressing_mode: &AddressingMode) {}

    fn bit(&mut self, addressing_mode: &AddressingMode) {}

    fn bmi(&mut self, addressing_mode: &AddressingMode) {}

    fn bne(&mut self, addressing_mode: &AddressingMode) {}

    fn bpl(&mut self, addressing_mode: &AddressingMode) {}

    fn bvc(&mut self, addressing_mode: &AddressingMode) {}

    fn bvs(&mut self, addressing_mode: &AddressingMode) {}

    fn clc(&mut self) {
        self.status.change_carry_flag(false);
    }

    fn cld(&mut self) {
        self.status.change_decimal_mode_flag(false);
    }

    fn cli(&mut self) {
        self.status.change_interrupt_disable_flag(false);
    }

    fn clv(&mut self) {
        self.status.change_overflow_flag(false);
    }

    fn cmp(&mut self, addressing_mode: &AddressingMode) {}

    fn cpx(&mut self, addressing_mode: &AddressingMode) {}

    fn cpy(&mut self, addressing_mode: &AddressingMode) {}

    fn dec(&mut self, addressing_mode: &AddressingMode) {}

    fn dex(&mut self) {
        self.register_x.dec();
        self.status.change_zero_flag(self.register_x.get());
        self.status.change_negative_flag(self.register_x.get());
    }

    fn dey(&mut self) {
        self.register_y.dec();
        self.status.change_zero_flag(self.register_y.get());
        self.status.change_negative_flag(self.register_y.get());
    }

    fn eor(&mut self, addressing_mode: &AddressingMode) {}

    fn inx(&mut self) {
        self.register_x.inc();
        self.status.change_zero_flag(self.register_x.get());
        self.status.change_negative_flag(self.register_x.get());
    }

    fn inc(&mut self, addressing_mode: &AddressingMode) {}

    fn iny(&mut self) {
        self.register_y.inc();
        self.status.change_zero_flag(self.register_y.get());
        self.status.change_negative_flag(self.register_y.get());
    }

    fn jmp(&mut self, addressing_mode: &AddressingMode) {}

    fn jsr(&mut self, addressing_mode: &AddressingMode) {}

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_dereferenced_value(addressing_mode);
        self.accumulator.set(value);
        self.status.change_zero_flag(value);
        self.status.change_negative_flag(value);
    }

    fn ldx(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_dereferenced_value(addressing_mode);
        self.register_x.set(value);
        self.status.change_zero_flag(value);
        self.status.change_negative_flag(value);
    }

    fn ldy(&mut self, addressing_mode: &AddressingMode) {
        let value = self.get_dereferenced_value(addressing_mode);
        self.register_y.set(value);
        self.status.change_zero_flag(value);
        self.status.change_negative_flag(value);
    }

    fn lsr(&mut self, addressing_mode: &AddressingMode) {}

    fn nop(&mut self) {
        self.program_counter.inc();
    }

    fn ora(&mut self, addressing_mode: &AddressingMode) {}

    fn pha(&mut self) -> Result<(), StackError> {
        self.stack.push(self.accumulator.get(), &mut self.memory)?;
        Ok(())
    }

    fn php(&mut self) -> Result<(), StackError> {
        self.stack.push(self.status.get(), &mut self.memory)?;
        Ok(())
    }

    fn pla(&mut self) -> Result<(), StackError> {
        let value = self.stack.pull(&mut self.memory)?;
        self.accumulator.set(value);
        self.status.change_zero_flag(value);
        self.status.change_negative_flag(value);
        Ok(())
    }

    fn plp(&mut self) -> Result<(), StackError> {
        let value = self.stack.pull(&mut self.memory)?;
        self.status.set(value);
        Ok(())
    }

    fn rol(&mut self, addressing_mode: &AddressingMode) {}

    fn ror(&mut self, addressing_mode: &AddressingMode) {}

    fn rti(&mut self, addressing_mode: &AddressingMode) {}

    fn rts(&mut self, addressing_mode: &AddressingMode) {}

    fn sbc(&mut self, addressing_mode: &AddressingMode) {}

    fn sec(&mut self, addressing_mode: &AddressingMode) {}

    fn sed(&mut self, addressing_mode: &AddressingMode) {}

    fn sei(&mut self, addressing_mode: &AddressingMode) {}

    fn sta(&mut self, addressing_mode: &AddressingMode) {}

    fn stx(&mut self, addressing_mode: &AddressingMode) {}

    fn sty(&mut self, addressing_mode: &AddressingMode) {}

    fn tax(&mut self) {
        self.register_x.set(self.accumulator.get());
        self.status.change_zero_flag(self.register_x.get());
        self.status.change_negative_flag(self.register_x.get());
    }

    fn tay(&mut self) {
        self.register_y.set(self.accumulator.get());
        self.status.change_zero_flag(self.register_y.get());
        self.status.change_negative_flag(self.register_y.get());
    }

    fn tsx(&mut self) {
        self.register_x.set(self.stack.get_pointer());
        self.status.change_zero_flag(self.register_x.get());
        self.status.change_negative_flag(self.register_x.get());
    }

    fn txa(&mut self) {
        self.accumulator.set(self.register_x.get());
        self.status.change_zero_flag(self.accumulator.get());
        self.status.change_negative_flag(self.accumulator.get());
    }

    fn txs(&mut self) -> Result<(), StackError> {
        let new_pointer = self.register_x.get();
        self.stack.set_pointer(new_pointer)?;
        self.status.change_zero_flag(new_pointer);
        self.status.change_negative_flag(new_pointer);
        Ok(())
    }

    fn tya(&mut self) {
        self.accumulator.set(self.register_y.get());
        self.status.change_zero_flag(self.accumulator.get());
        self.status.change_negative_flag(self.accumulator.get());
    }

    fn next_instruction(&mut self) -> Result<&'static Instruction, UnknownOpCode> {
        let opcode = self.memory.read(self.program_counter.get());
        self.program_counter.inc();
        OPCODES.get(&opcode).ok_or(UnknownOpCode(opcode))
    }

    fn get_dereferenced_value(&mut self, addressing_mode: &AddressingMode) -> u8 {
        let value = match addressing_mode {
            AddressingMode::Absolute => {
                let value_address = self.memory.read(self.program_counter.get());
                self.memory.read(value_address)
            }
            AddressingMode::AbsoluteX => {
                let value_address: u16 = self.memory.read(self.program_counter.get());
                self.memory
                    .read(value_address.wrapping_add(self.register_x.get() as u16))
            }
            AddressingMode::AbsoluteY => {
                let value_address: u16 = self.memory.read(self.program_counter.get());
                self.memory
                    .read(value_address.wrapping_add(self.register_y.get() as u16))
            }
            AddressingMode::Immediate | AddressingMode::Relative => {
                self.memory.read(self.program_counter.get())
            }
            AddressingMode::IndexedIndirectX => {
                let value_address: u8 = self.memory.read(self.program_counter.get());
                let indirect_address = self
                    .memory
                    .read(value_address.wrapping_add(self.register_x.get()) as u16);
                self.memory.read(indirect_address)
            }
            AddressingMode::Indirect => {
                let value_address = self.memory.read(self.program_counter.get());
                let indirect_address = self.memory.read(value_address);
                self.memory.read(indirect_address)
            }
            AddressingMode::IndirectIndexedY => {
                let value_address = self.memory.read(self.program_counter.get());
                let indirect_address: u16 = self.memory.read(value_address);
                self.memory
                    .read(indirect_address.wrapping_add(self.register_y.get() as u16))
            }
            AddressingMode::ZeroPage => {
                let address: u8 = self.memory.read(self.program_counter.get());
                self.memory.read(address as u16)
            }
            AddressingMode::ZeroPageX => {
                let address: u8 = self.memory.read(self.program_counter.get());
                self.memory
                    .read(address.wrapping_add(self.register_x.get()) as u16)
            }
            AddressingMode::ZeroPageY => {
                let address: u8 = self.memory.read(self.program_counter.get());
                self.memory
                    .read(address.wrapping_add(self.register_y.get()) as u16)
            }
            // TODO: Add error instead of panic
            AddressingMode::Accumulator | AddressingMode::Implied => {
                panic!("Mode {addressing_mode:?} can't have operand address")
            }
        };
        self.program_counter
            .add(addressing_mode.operand_bytes() as u16);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_loads_value_to_register_a_and_change_status() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xa9, 0x0f, 0x00]);
        assert_eq!(cpu.accumulator, 0x0f);
        assert_eq!(cpu.status, 0);

        cpu.run(vec![0xa9, 0, 0x00]);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.status, 0x02);

        cpu.run(vec![0xa9, 0xff, 0x00]);
        assert_eq!(cpu.accumulator, 0xff);
        assert_eq!(cpu.status, 0x80);
    }

    #[test]
    fn test_tax_transfer_from_register_a_to_x_and_change_status() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xa9, 0x0f, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x0f);
        assert_eq!(cpu.accumulator, 0x0f);
        assert_eq!(cpu.status, 0);

        cpu.run(vec![0xa9, 0, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.status, 0x02);

        cpu.run(vec![0xa9, 0xff, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0xff);
        assert_eq!(cpu.accumulator, 0xff);
        assert_eq!(cpu.status, 0x80);
    }

    #[test]
    fn test_inx_increments_register_x() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x01);
        assert_eq!(cpu.status, 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }
}
