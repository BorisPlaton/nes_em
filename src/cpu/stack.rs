use crate::cpu::error::StackError;
use crate::cpu::register::register::Register;
use crate::mem::map::{IOOperation, MemoryMap};

pub struct Stack {
    stack_pointer: Register<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_pointer: Register::new(0x01FF),
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

    pub fn push(&mut self, value: u8, memory_map: &mut MemoryMap) -> Result<(), StackError> {
        let address = self.stack_pointer.get();
        if address > 0x0100 {
            return Err(StackError::Overflow);
        }
        self.stack_pointer.dec();
        memory_map.write(address, value);
        Ok(())
    }

    pub fn pull(&mut self, memory_map: &mut MemoryMap) -> Result<u8, StackError> {
        if self.stack_pointer.get() == 0x01FF {
            return Err(StackError::Underflow);
        }
        Ok(memory_map.read(self.stack_pointer.inc()))
    }
}
