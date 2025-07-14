use crate::cpu::error::UnknownOpCode;
use crate::cpu::flags::Flags;
use crate::cpu::opcode::{AddressingMode, Instruction, OpCode};

pub struct CPU {
    pub accumulator: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub flags: Flags,
    pub program_counter: u16,
    memory: [u8; 0xFFFF],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            register_x: 0,
            register_y: 0,
            flags: Flags::new(),
            program_counter: 0,
            memory: [0; 0xFFFF],
        }
    }

    pub fn run(&mut self, program: Vec<u8>) {
        self.load_program(program);
        self.reset();
        // TODO: Add proper handling of errors that may occur
        self.interpret().unwrap();
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].clone_from_slice(&program);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn reset(&mut self) {
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.accumulator = 0;
        self.register_x = 0;
        self.flags.reset();
    }

    pub fn interpret(&mut self) -> Result<(), UnknownOpCode> {
        loop {
            let instruction: Instruction = self.next_instruction().try_into()?;
            match instruction.opcode {
                OpCode::LDA => self.lda(&instruction.mode),
                OpCode::TAX => self.tax(),
                OpCode::INX => self.inx(),
                OpCode::BRK => return Ok(()),
            }
        }
    }

    fn lda(&mut self, addressing_mode: &AddressingMode) {
        self.accumulator = self.get_operand_value(addressing_mode);
        self.flags.change_zero_flag(self.accumulator);
        self.flags.change_negative_flag(self.accumulator);
    }

    fn tax(&mut self) {
        self.register_x = self.accumulator;
        self.flags.change_zero_flag(self.register_x);
        self.flags.change_negative_flag(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.flags.change_zero_flag(self.register_x);
        self.flags.change_negative_flag(self.register_x);
    }

    fn next_instruction(&mut self) -> u8 {
        let opcode = self.mem_read(self.program_counter);
        self.program_counter += 1;

        opcode
    }

    fn get_operand_value(&mut self, addressing_mode: &AddressingMode) -> u8 {
        let value = match addressing_mode {
            AddressingMode::Absolute => {
                let address = self.mem_read_u16(self.program_counter);
                self.mem_read(address)
            }
            AddressingMode::AbsoluteX => {
                let address = self
                    .mem_read_u16(self.program_counter)
                    .wrapping_add(self.register_x as u16);
                self.mem_read(address)
            }
            AddressingMode::AbsoluteY => {
                let address = self
                    .mem_read_u16(self.program_counter)
                    .wrapping_add(self.register_y as u16);
                self.mem_read(address)
            }
            AddressingMode::Immediate => self.mem_read(self.program_counter),
            // TODO: Add error instead of panic
            AddressingMode::NoneAddressing => panic!("Mode can't have operand address"),
            AddressingMode::IndexedIndirectX => {
                let address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x) as u16;
                let indirect_address = self.mem_read_u16(address);
                self.mem_read(indirect_address)
            }
            AddressingMode::IndexedIndirectY => {
                let address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_y) as u16;
                let indirect_address = self.mem_read_u16(address);
                self.mem_read(indirect_address)
            }
            AddressingMode::ZeroPage => {
                let address = self.mem_read(self.program_counter);
                self.mem_read(address as u16)
            }
            AddressingMode::ZeroPageX => {
                let address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_x);
                self.mem_read(address as u16)
            }
            AddressingMode::ZeroPageY => {
                let address = self
                    .mem_read(self.program_counter)
                    .wrapping_add(self.register_y);
                self.mem_read(address as u16)
            }
            _ => todo!(),
        };
        self.program_counter += addressing_mode.operand_bytes() as u16;
        value
    }

    fn mem_read_u16(&mut self, address: u16) -> u16 {
        u16::from_le_bytes([self.mem_read(address), self.mem_read(address + 1)])
    }

    fn mem_write_u16(&mut self, address: u16, value: u16) {
        let value_le_bytes: [u8; 2] = value.to_le_bytes();
        self.mem_write(address, value_le_bytes[0]);
        self.mem_write(address + 1, value_le_bytes[1]);
    }

    fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data
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
        assert_eq!(cpu.flags, 0);

        cpu.run(vec![0xa9, 0, 0x00]);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.flags, 0x02);

        cpu.run(vec![0xa9, 0xff, 0x00]);
        assert_eq!(cpu.accumulator, 0xff);
        assert_eq!(cpu.flags, 0x80);
    }

    #[test]
    fn test_tax_transfer_from_register_a_to_x_and_change_status() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xa9, 0x0f, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0x0f);
        assert_eq!(cpu.accumulator, 0x0f);
        assert_eq!(cpu.flags, 0);

        cpu.run(vec![0xa9, 0, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.accumulator, 0);
        assert_eq!(cpu.flags, 0x02);

        cpu.run(vec![0xa9, 0xff, 0xaa, 0x00]);
        assert_eq!(cpu.register_x, 0xff);
        assert_eq!(cpu.accumulator, 0xff);
        assert_eq!(cpu.flags, 0x80);
    }

    #[test]
    fn test_inx_increments_register_x() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0x01);
        assert_eq!(cpu.flags, 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }
}
