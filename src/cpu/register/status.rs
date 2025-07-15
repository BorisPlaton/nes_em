use crate::cpu::register::register::Register;

pub struct Status {
    state: Register<u8>,
}

impl Status {
    pub fn new() -> Self {
        Status {
            state: Register::<u8>::new(0b0001_0000),
        }
    }

    pub fn set(&mut self, value: u8) {
        self.state.set(value);
    }

    pub fn get(&self) -> u8 {
        self.state.get()
    }

    pub fn reset(&mut self) {
        self.state.set(0b0001_0000);
    }

    pub fn change_carry_flag(&mut self, activate: bool) {
        self.change_flag(0b0000_0001, activate);
    }

    pub fn change_zero_flag(&mut self, value: u8) {
        self.change_flag(0b0000_0010, value == 0);
    }

    pub fn change_interrupt_disable_flag(&mut self, activate: bool) {
        self.change_flag(0b0000_0100, activate);
    }

    pub fn change_decimal_mode_flag(&mut self, activate: bool) {
        self.change_flag(0b0000_1000, activate);
    }

    pub fn change_break_command_flag(&mut self, activate: bool) {
        self.change_flag(0b0010_0000, activate);
    }

    pub fn change_overflow_flag(&mut self, activate: bool) {
        self.change_flag(0b0100_0000, activate);
    }

    pub fn change_negative_flag(&mut self, value: u8) {
        self.change_flag(0b1000_0000, value & 0x10 != 0);
    }

    fn change_flag(&mut self, flag: u8, activate: bool) {
        let new_state = if activate {
            self.state.get() | flag
        } else {
            self.state.get() & !flag
        };
        self.state.set(new_state);
    }
}
