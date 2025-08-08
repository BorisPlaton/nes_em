// PPUADDR - VRAM address ($2006 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUADDR
//
// 1st write  2nd write
// 15 bit  8  7  bit  0
// ---- ----  ---- ----
// ..AA AAAA  AAAA AAAA
//   || ||||  |||| ||||
//   ++-++++--++++-++++- VRAM address
pub struct PPUADDR {
    value: u16,
}

impl PPUADDR {
    pub fn new() -> PPUADDR {
        PPUADDR { value: 0 }
    }

    pub fn read(&self) -> u16 {
        self.value
    }

    pub fn write(&mut self, value: u8, register_w: &mut bool) {
        let mut value_bytes: [u8; 2] = self.value.to_be_bytes();
        let register_w_value = *register_w;
        if register_w_value {
            value_bytes[0] = value;
        } else {
            value_bytes[1] = value;
        }
        self.set(u16::from_be_bytes(value_bytes));
        *register_w = !register_w_value;
    }

    pub fn inc(&mut self, value: u8) {
        self.set(self.value.wrapping_add(value as u16));
    }

    fn set(&mut self, value: u16) {
        self.value = value & 0b0011_1111_1111_1111;
    }
}
