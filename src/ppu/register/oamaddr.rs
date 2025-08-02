// OAMADDR - Sprite RAM address ($2003 write)
// https://www.nesdev.org/wiki/PPU_registers#OAMADDR
//
// 7  bit  0
// ---- ----
// AAAA AAAA
// |||| ||||
// ++++-++++- OAM address
pub struct OAMADDR {
    data: u8,
}

impl OAMADDR {
    pub fn new() -> Self {
        OAMADDR { data: 0 }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, data: u8) {
        self.data = data;
    }

    pub fn inc(&mut self) {
        self.data = self.data.wrapping_add(1);
    }
}
