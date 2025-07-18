use crate::cpu::register::register::Register;

pub struct Status {
    state: Register<u8>,
}

impl Status {
    pub fn new() -> Self {
        Status {
            state: Register::<u8>::new(0b0010_0000),
        }
    }

    pub fn set(&mut self, value: u8) {
        self.state.set(value & 0b1110_1111);
    }

    pub fn get(&self) -> u8 {
        self.state.get()
    }

    pub fn get_carry_flag(&self) -> u8 {
        self.state.get() & 1
    }

    pub fn reset(&mut self) {
        self.state.set(0b0010_0000);
    }

    pub fn is_carry_flag_set(&self) -> bool {
        self.state.get() & 0b0000_0001 != 0
    }

    pub fn is_zero_flag_set(&self) -> bool {
        self.state.get() & 0b0000_0010 != 0
    }

    pub fn is_overflow_flag_set(&self) -> bool {
        self.state.get() & 0b0100_0000 != 0
    }

    pub fn is_negative_flag_set(&self) -> bool {
        self.state.get() & 0b1000_0000 != 0
    }

    pub fn set_carry_flag_to(&mut self, activate: bool) {
        self.change_flag(0b0000_0001, activate);
    }

    pub fn set_interrupt_disable_flag_to(&mut self, activate: bool) {
        self.change_flag(0b0000_0100, activate);
    }

    pub fn set_decimal_mode_flag_to(&mut self, activate: bool) {
        self.change_flag(0b0000_1000, activate);
    }

    pub fn set_overflow_flag_to(&mut self, activate: bool) {
        self.change_flag(0b0100_0000, activate);
    }

    pub fn set_negative_flag(&mut self, value: u8) {
        self.change_flag(0b1000_0000, value & 0x10 != 0);
    }

    pub fn set_zero_flag(&mut self, value: u8) {
        self.change_flag(0b0000_0010, value == 0);
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
