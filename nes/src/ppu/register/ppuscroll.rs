// PPUSCROLL - X and Y scroll ($2005 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUSCROLL
//
// 1st write
// 7654 3210 bit
// ---- ----
// XXXX XXXX
// |||| ||||
// ++++-++++- X scroll bits 7-0 (bit 8 in PPUCTRL bit 0)
//
// 2nd write
// 7654 3210 bit
// ---- ----
// YYYY YYYY
// |||| ||||
// ++++-++++- Y scroll bits 7-0 (bit 8 in PPUCTRL bit 1)
pub struct PPUSCROLL {
    data: (u8, u8),
    latch: bool,
}

impl PPUSCROLL {
    pub fn new() -> PPUSCROLL {
        PPUSCROLL {
            data: (0, 0),
            latch: false,
        }
    }

    pub fn write(&mut self, value: u8) {
        self.latch = !self.latch;
        if self.latch {
            self.data.0 = value;
        } else {
            self.data.1 = value
        }
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }
}
