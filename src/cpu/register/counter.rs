use crate::cpu::register::register::Register;

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

    pub fn move_with_offset(&mut self, value: u8) {
        self.register.add_signed(value as i8 as i16);
    }
}
