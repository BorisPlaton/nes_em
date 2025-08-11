use crate::cpu::bus::{CPUBus, CPUBusOperation};
use crate::cpu::register::register::Register;

pub struct Stack {
    stack_pointer: Register<u8>,
}

impl Stack {
    const INITIAL_STACK_POINTER: u8 = 0xFD;
    const STACK_ADDR: u16 = 0x0100;

    pub fn new() -> Stack {
        Stack {
            stack_pointer: Register::new(Stack::INITIAL_STACK_POINTER),
        }
    }

    pub fn get_pointer(&self) -> u8 {
        self.stack_pointer.get()
    }

    pub fn reset(&mut self) {
        self.stack_pointer.set(Stack::INITIAL_STACK_POINTER);
    }

    pub fn set_pointer(&mut self, value: u8) {
        self.stack_pointer.set(value);
    }

    pub fn get_stack_address(&self) -> u16 {
        Stack::STACK_ADDR + self.stack_pointer.get() as u16
    }
}

pub trait StackOperation<T> {
    fn push(&mut self, value: T, bus: &mut CPUBus);

    fn pull(&mut self, bus: &mut CPUBus) -> T;
}

impl StackOperation<u8> for Stack {
    fn push(&mut self, value: u8, bus: &mut CPUBus) {
        bus.write(self.get_stack_address(), value);
        self.stack_pointer.dec();
    }

    fn pull(&mut self, bus: &mut CPUBus) -> u8 {
        self.stack_pointer.inc();
        bus.read(self.get_stack_address())
    }
}

impl StackOperation<u16> for Stack {
    fn push(&mut self, value: u16, bus: &mut CPUBus) {
        let value_bytes: [u8; 2] = value.to_be_bytes();
        StackOperation::<u8>::push(self, value_bytes[0], bus);
        StackOperation::<u8>::push(self, value_bytes[1], bus);
    }

    fn pull(&mut self, bus: &mut CPUBus) -> u16 {
        let lo_byte = StackOperation::<u8>::pull(self, bus);
        let hi_byte = StackOperation::<u8>::pull(self, bus);
        u16::from_le_bytes([lo_byte, hi_byte])
    }
}
