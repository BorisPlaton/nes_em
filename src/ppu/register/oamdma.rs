// OAMDMA - Sprite DMA ($4014 write)
// https://www.nesdev.org/wiki/PPU_registers#OAMDMA_-_Sprite_DMA_($4014_write)
//
// 7  bit  0
// ---- ----
// AAAA AAAA
// |||| ||||
// ++++-++++- Source page (high byte of source address)
pub struct OAMDMA {
    value: u8,
}

impl OAMDMA {
    pub fn new() -> OAMDMA {
        OAMDMA { value: 0 }
    }

    pub fn write(&mut self, value: u8) {
        self.value = value;
    }
}
