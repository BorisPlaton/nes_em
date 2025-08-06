use crate::cpu::bus::{CPUBus, CPUBusOperation};
use crate::cpu::error::StackError;
use crate::cpu::register::register::Register;

const INITIAL_STACK_POINTER: u16 = 0x01FD;

pub struct Stack {
    stack_pointer: Register<u16>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack_pointer: Register::new(INITIAL_STACK_POINTER),
        }
    }

    pub fn get_pointer(&self) -> u8 {
        self.stack_pointer.get() as u8
    }

    pub fn reset(&mut self) {
        self.stack_pointer.set(INITIAL_STACK_POINTER);
    }

    pub fn set_pointer(&mut self, value: u8) -> Result<(), StackError> {
        if !(0x01 < value && value < 0xFF) {
            return Err(StackError::OutOfStackRange(value));
        }
        self.stack_pointer.set(0x0100 + value as u16);
        Ok(())
    }
}

pub trait StackOperation<T> {
    fn push(&mut self, value: T, bus: &mut CPUBus) -> Result<(), StackError>;

    fn pull(&mut self, bus: &mut CPUBus) -> Result<T, StackError>;
}

impl StackOperation<u8> for Stack {
    fn push(&mut self, value: u8, bus: &mut CPUBus) -> Result<(), StackError> {
        let address = self.stack_pointer.get();
        if address < 0x0100 {
            return Err(StackError::Overflow);
        }
        self.stack_pointer.dec();
        bus.write(address, value);
        Ok(())
    }

    fn pull(&mut self, bus: &mut CPUBus) -> Result<u8, StackError> {
        if self.stack_pointer.get() == INITIAL_STACK_POINTER {
            return Err(StackError::Underflow);
        }
        Ok(bus.read(self.stack_pointer.inc()))
    }
}

impl StackOperation<u16> for Stack {
    fn push(&mut self, value: u16, bus: &mut CPUBus) -> Result<(), StackError> {
        let value_bytes: [u8; 2] = value.to_be_bytes();
        StackOperation::<u8>::push(self, value_bytes[0], bus)?;
        StackOperation::<u8>::push(self, value_bytes[1], bus)?;
        Ok(())
    }

    fn pull(&mut self, bus: &mut CPUBus) -> Result<u16, StackError> {
        let lo_byte = StackOperation::<u8>::pull(self, bus)?;
        let hi_byte = StackOperation::<u8>::pull(self, bus)?;
        Ok(u16::from_le_bytes([lo_byte, hi_byte]))
    }
}
