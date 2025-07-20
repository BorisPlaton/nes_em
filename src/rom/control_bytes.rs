pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(PartialEq)]
pub enum NESFormat {
    NES1,
    NES2,
}

pub struct ControlBytes {
    byte1: u8,
    byte2: u8,
}

impl ControlBytes {
    pub fn new(byte1: u8, byte2: u8) -> ControlBytes {
        ControlBytes {
            byte1: byte1,
            byte2: byte2,
        }
    }

    pub fn mirroring(&self) -> Mirroring {
        match self.byte1 & 0b0000_1001 {
            0b0000_1001 | 0b0000_1000 => Mirroring::FourScreen,
            0b0000_0001 => Mirroring::Vertical,
            _ => Mirroring::Horizontal,
        }
    }

    pub fn trainer_size(&self) -> usize {
        if self.byte1 & 0b0000_0100 != 0 {
            512
        } else {
            0
        }
    }

    pub fn mapper(&self) -> u8 {
        self.byte2 & 0b1111_0000 + ((self.byte1 & 0b1111_0000) >> 4)
    }

    pub fn nes_format(&self) -> NESFormat {
        if self.byte2 & 0b0000_1000 != 0 {
            NESFormat::NES2
        } else {
            NESFormat::NES1
        }
    }
}
