use crate::error::InterpreterError;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) -> Result<(), InterpreterError> {
        self.program_counter = 0;

        loop {
            match self.next(&program)? {
                // LDA opcode
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
                0xA9 => self.lda(&program)?,
                // TAX opcode
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#TAX
                0xAA => self.tax(),
                // INX opcode
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#INX
                0xE8 => self.inx(),
                // BRK opcode
                // https://www.nesdev.org/obelisk-6502-guide/reference.html#BRK
                0x00 => {
                    return Ok(());
                }
                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, program: &[u8]) -> Result<(), InterpreterError> {
        let param = self.next(&program)?;

        self.register_a = *param;
        self.set_zero_and_negative_flags(self.register_a);

        Ok(())
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.set_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.set_zero_and_negative_flags(self.register_x);
    }

    fn set_zero_and_negative_flags(&mut self, result: u8) {
        self.status = if result == 0 {
            self.status | 0b0000_0010
        } else {
            self.status & 0b1111_1101
        };

        self.status = if result & 0b1000_0000 != 0 {
            self.status | 0b1000_0000
        } else {
            self.status & 0b0111_1111
        };
    }

    fn next<'a>(&mut self, program: &'a [u8]) -> Result<&'a u8, InterpreterError> {
        let opcode = program
            .get(self.program_counter as usize)
            .ok_or(InterpreterError::InvalidPC(self.program_counter as usize))?;
        self.program_counter += 1;

        Ok(opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lda_loads_value_to_register_a_and_change_status() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xa9, 0x0f, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0x0f);
        assert_eq!(cpu.status, 0);

        cpu.interpret(vec![0xa9, 0, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status, 0x02);

        cpu.interpret(vec![0xa9, 0xff, 0x00]).unwrap();
        assert_eq!(cpu.register_a, 0xff);
        assert_eq!(cpu.status, 0x80);
    }

    #[test]
    fn test_tax_transfer_from_register_a_to_x_and_change_status() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xa9, 0x0f, 0xaa, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0x0f);
        assert_eq!(cpu.register_a, 0x0f);
        assert_eq!(cpu.status, 0);

        cpu.interpret(vec![0xa9, 0, 0xaa, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0);
        assert_eq!(cpu.register_a, 0);
        assert_eq!(cpu.status, 0x02);

        cpu.interpret(vec![0xa9, 0xff, 0xaa, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0xff);
        assert_eq!(cpu.register_a, 0xff);
        assert_eq!(cpu.status, 0x80);
    }

    #[test]
    fn test_inx_increments_register_x() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xe8, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0x01);
        assert_eq!(cpu.status, 0);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();

        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();

        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]).unwrap();
        assert_eq!(cpu.register_x, 0xc1)
    }
}