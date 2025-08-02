// PPUSCROLL - X and Y scroll ($2005 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUSCROLL
//
// 1st write
// 7  bit  0
// ---- ----
// XXXX XXXX
// |||| ||||
// ++++-++++- X scroll bits 7-0 (bit 8 in PPUCTRL bit 0)
//
// 2nd write
// 7  bit  0
// ---- ----
// YYYY YYYY
// |||| ||||
// ++++-++++- Y scroll bits 7-0 (bit 8 in PPUCTRL bit 1)
pub struct PPUSCROLL {
    data: (u8, u8),
}

impl PPUSCROLL {
    pub fn new() -> PPUSCROLL {
        PPUSCROLL { data: (0, 0) }
    }

    pub fn write(&mut self, value: u8, register_w: &mut bool) {
        let register_w_value = *register_w;
        if register_w_value {
            self.data.0 = value;
        } else {
            self.data.1 = value
        }
        *register_w = !register_w_value;
    }
}
