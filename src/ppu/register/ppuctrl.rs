use bitflags::{Flags, bitflags};

// PPUCTRL - Miscellaneous settings ($2000 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUCTRL
//
// 7654 3210 bit
// ---- ----
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Vblank NMI enable (0: off, 1: on)
bitflags! {
    pub struct PPUCTRL: u8 {
        const NAMETABLE_ADDR_1 = 0b0000_0001;
        const NAMETABLE_ADDR_2 = 0b0000_0010;
        const ADDR_INCREMENT = 0b0000_0100;
        const SPRITE_ADDR = 0b0000_1000;
        const BACKGROUND_ADDR = 0b0001_0000;
        const SPRITE_SIZE = 0b0010_0000;
        const MASTER_SLAVE_SELECT =  0b0100_0000;
        const NMI_ENABLE = 0b1000_0000;
    }
}

impl PPUCTRL {
    pub fn new() -> Self {
        PPUCTRL::from_bits_truncate(0)
    }

    pub fn address_increment(&self) -> u8 {
        if self.contains(PPUCTRL::ADDR_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn write(&mut self, value: u8) {
        *self = PPUCTRL::from_bits_truncate(value);
    }

    pub fn is_vblank_nmi_set(&self) -> bool {
        self.contains(PPUCTRL::NMI_ENABLE)
    }
}
