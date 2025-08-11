use crate::cpu::register::register::Register;

type PageCrossed = bool;

pub struct ProgramCounter {
    register: Register<u16>,
}

impl ProgramCounter {
    pub fn new() -> ProgramCounter {
        ProgramCounter {
            register: Register::new(0),
        }
    }

    pub fn get(&self) -> u16 {
        self.register.get()
    }

    pub fn set(&mut self, value: u16) {
        self.register.set(value);
    }

    pub fn inc(&mut self) {
        self.register.inc();
    }

    pub fn add(&mut self, value: u16) {
        let value = self.register.add(value);
        self.register.set(value);
    }

    pub fn move_with_offset(&mut self, value: u8) -> PageCrossed {
        let previous_val = self.register.get();
        let current_value = self.register.add_signed(value as i8 as i16);
        previous_val.to_be_bytes()[0] != current_value.to_be_bytes()[0]
    }
}
