pub struct Flags {
    state: u8,
}

impl Flags {
    pub fn new() -> Self {
        Flags { state: 0b0001_0000 }
    }

    pub fn reset(&mut self) {
        self.state = 0b0001_0000;
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
        if activate {
            self.state |= flag
        } else {
            self.state &= !flag
        };
    }
}
