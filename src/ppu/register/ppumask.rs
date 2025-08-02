use bitflags::bitflags;

// PPUMASK - Rendering settings ($2001 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUMASK
//
// 7  bit  0
// ---- ----
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: greyscale)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Enable background rendering
// |||+------ 1: Enable sprite rendering
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue
bitflags! {
    pub struct PPUMASK: u8 {
        const GREYSCALE = 0b00000001;
        const SHOW_BG_LEFT_8_PX = 0b00000010;
        const SHOW_SPRITES_LEFT_8_PX = 0b00000100;
        const ENABLE_BG_RENDERING = 0b00001000;
        const ENABLE_SPRITE_RENDERING = 0b00010000;
        const EMPHASIZE_RED = 0b00100000;
        const EMPHASIZE_GREEN = 0b01000000;
        const EMPHASIZE_BLUE = 0b10000000;
    }
}

impl PPUMASK {
    pub fn new() -> Self {
        PPUMASK::from_bits_truncate(0)
    }

    pub fn write(&mut self, value: u8) {
        *self = PPUMASK::from_bits_truncate(value);
    }
}
