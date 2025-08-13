use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone)]
    pub struct JoypadRegister: u8 {
        const BUTTON_A = 0b0000_0001;
        const BUTTON_B = 0b0000_0010;
        const SELECT = 0b0000_0100;
        const START = 0b0000_1000;
        const UP = 0b0001_0000;
        const DOWN = 0b0010_0000;
        const LEFT = 0b0100_0000;
        const RIGHT = 0b1000_0000;
    }
}

impl JoypadRegister {
    pub fn new() -> JoypadRegister {
        JoypadRegister::from_bits_truncate(0)
    }

    pub fn get_button_state(&self, button_index: u8) -> u8 {
        if button_index >= 8 {
            1
        } else {
            (self.bits() & (1 << button_index)) >> button_index
        }
    }
}
