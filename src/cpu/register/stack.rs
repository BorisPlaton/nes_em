use crate::cpu::bus::{CPUBus, IOOperation};
use crate::cpu::error::StackError;
use crate::cpu::register::register::Register;

pub struct Stack {
    stack_pointer: Register<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_pointer: Register::new(0x01FD),
        }
    }

    pub fn get_pointer(&self) -> u8 {
        self.stack_pointer.get() as u8
    }

    pub fn set_pointer(&mut self, value: u8) -> Result<(), StackError> {
        if !(0x01 < value && value < 0xFF) {
            return Err(StackError::OutOfStackRange(value));
        }
        self.stack_pointer.set(0x0100 + value as u16);
        Ok(())
    }

    pub fn push(&mut self, value: u8, bus: &mut CPUBus) -> Result<(), StackError> {
        let address = self.stack_pointer.get();
        if address < 0x0100 {
            return Err(StackError::Overflow);
        }
        self.stack_pointer.dec();
        bus.write(address, value);
        Ok(())
    }

    pub fn pull(&mut self, bus: &mut CPUBus) -> Result<u8, StackError> {
        if self.stack_pointer.get() == 0x01FF {
            return Err(StackError::Underflow);
        }
        Ok(bus.read(self.stack_pointer.inc()))
    }
}
