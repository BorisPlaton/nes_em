// OAMDATA - Sprite RAM data ($2004 read/write)
// https://www.nesdev.org/wiki/PPU_registers#OAMDATA_-_Sprite_RAM_data_($2004_read/write)
//
// 7  bit  0
// ---- ----
// DDDD DDDD
// |||| ||||
// ++++-++++- OAM data
pub struct OAMDATA {
    data: u8,
}

impl OAMDATA {
    pub fn new() -> Self {
        OAMDATA { data: 0 }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn write(&mut self, data: u8) {
        self.data = data;
    }
}
