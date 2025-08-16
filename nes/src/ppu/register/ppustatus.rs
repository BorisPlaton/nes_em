use bitflags::bitflags;

// PPUSTATUS - Rendering events ($2002 read)
// https://www.nesdev.org/wiki/PPU_registers#PPUSTATUS
//
// 7654 3210 bit
// ---- ----
// VSOx xxxx
// |||| ||||
// |||+-++++- (PPU open bus or 2C05 PPU identifier)
// ||+------- Sprite overflow flag
// |+-------- Sprite 0 hit flag
// +--------- Vblank flag, cleared on read. Unreliable;
bitflags! {
    pub struct PPUSTATUS: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT_FLAG =  0b0100_0000;
        const VBLANK_FLAG = 0b1000_0000;
    }
}

impl PPUSTATUS {
    pub fn new() -> Self {
        PPUSTATUS::from_bits_truncate(0b0)
    }

    pub fn read(&self) -> u8 {
        self.bits()
    }
}
