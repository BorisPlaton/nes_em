use crate::controller::register::JoypadRegister;

// https://www.nesdev.org/wiki/Standard_controller
pub struct Controller {
    buttons: JoypadRegister,
    strobe: bool,
    button_index: u8,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            buttons: JoypadRegister::new(),
            strobe: false,
            button_index: 0,
        }
    }

    pub fn set_button_status(&mut self, button: JoypadRegister, status: bool) {
        self.buttons.set(button, status);
    }

    pub fn read(&mut self) -> u8 {
        let button_state = self.buttons.get_button_state(self.button_index);
        if !self.strobe && self.button_index <= 7 {
            self.button_index += 1;
        }
        button_state
    }

    pub fn write(&mut self, value: u8) {
        self.strobe = value & 1 == 1;
        if self.strobe {
            self.button_index = 0;
        }
    }
}
