use crate::ppu::mirroring::Mirroring;
use crate::ppu::register::oamaddr::OAMADDR;
use crate::ppu::register::oamdata::OAMDATA;
use crate::ppu::register::oamdma::OAMDMA;
use crate::ppu::register::ppuaddr::PPUADDR;
use crate::ppu::register::ppuctrl::PPUCTRL;
use crate::ppu::register::ppudata::PPUDATA;
use crate::ppu::register::ppumask::PPUMASK;
use crate::ppu::register::ppuscroll::PPUSCROLL;
use crate::ppu::register::ppustatus::PPUSTATUS;
use std::ops::Range;

pub struct PPU {
    // PPU Registers
    // https://www.nesdev.org/wiki/PPU_registers
    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    oamaddr: OAMADDR,
    oamdata: OAMDATA,
    ppuscroll: PPUSCROLL,
    ppuaddr: PPUADDR,
    ppudata: PPUDATA,
    oamdma: OAMDMA,

    chr_rom: Vec<u8>,
    mirroring: Mirroring,
    vram: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],

    pub scanline: u16,
    pub cycles: usize,
    nmi_interrupt: bool,
}

impl PPU {
    const CHR_ROM_START: u16 = 0x0000;
    const CHR_ROM_END: u16 = 0x1FFF;

    const VRAM_START: u16 = 0x2000;
    const VRAM_END: u16 = 0x2FFF;
    const VRAM_NAMETABLE_SIZE: u16 = 0x0400;

    const PALETTE_RAM_START: u16 = 0x3F00;
    const PALETTE_RAM_END: u16 = 0x3FFF;

    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            ppuctrl: PPUCTRL::new(),
            ppumask: PPUMASK::new(),
            ppustatus: PPUSTATUS::new(),
            oamaddr: OAMADDR::new(),
            oamdata: OAMDATA::new(),
            ppuscroll: PPUSCROLL::new(),
            ppuaddr: PPUADDR::new(),
            ppudata: PPUDATA::new(),
            oamdma: OAMDMA::new(),

            chr_rom,
            mirroring,
            vram: [0; 2048],
            palette_table: [0; 32],
            oam_data: [0; 256],

            scanline: 0,
            cycles: 0,
            nmi_interrupt: false,
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;

        if self.cycles < 341 {
            return false;
        }

        if self.is_sprite_0_hit(self.cycles) {
            self.ppustatus.set(PPUSTATUS::SPRITE_ZERO_HIT_FLAG, false);
        }

        self.cycles -= 341;
        self.scanline += 1;

        // https://www.nesdev.org/wiki/PPU_rendering#Vertical_blanking_lines_(241-260)
        if self.scanline == 241 {
            self.ppustatus.set(PPUSTATUS::VBLANK_FLAG, true);
            self.ppustatus.set(PPUSTATUS::SPRITE_ZERO_HIT_FLAG, false);
            self.nmi_interrupt = self.ppuctrl.contains(PPUCTRL::NMI_ENABLE);
        }

        if self.scanline >= 262 {
            self.scanline = 0;
            self.nmi_interrupt = false;
            self.ppustatus.set(PPUSTATUS::VBLANK_FLAG, false);
            self.ppustatus.set(PPUSTATUS::SPRITE_ZERO_HIT_FLAG, false);
            return true;
        }

        false
    }

    pub fn poll_nmi_interrupt(&mut self) -> bool {
        if self.nmi_interrupt {
            self.nmi_interrupt = false;
            true
        } else {
            false
        }
    }

    pub fn write_ppuctrl(&mut self, value: u8) {
        let nmi_disabled = !self.ppuctrl.contains(PPUCTRL::NMI_ENABLE);
        self.ppuctrl.write(value);
        self.nmi_interrupt = nmi_disabled
            && self.ppuctrl.contains(PPUCTRL::NMI_ENABLE)
            && self.ppustatus.contains(PPUSTATUS::VBLANK_FLAG);
    }

    pub fn write_ppumask(&mut self, value: u8) {
        self.ppumask.write(value);
    }

    pub fn write_oamaddr(&mut self, value: u8) {
        self.oamaddr.write(value);
    }

    pub fn write_oamdata(&mut self, value: u8) {
        self.oam_data[self.oamaddr.read() as usize] = value;
        self.oamaddr.inc()
    }

    pub fn write_ppuscroll(&mut self, value: u8) {
        self.ppuscroll.write(value);
    }

    pub fn write_ppuaddr(&mut self, address_part: u8) {
        self.ppuaddr.write(address_part);
    }

    pub fn write_ppudata(&mut self, value: u8) {
        let address = self.ppuaddr.read();

        match address {
            PPU::CHR_ROM_START..=PPU::CHR_ROM_END => self.chr_rom[address as usize] = value,
            PPU::VRAM_START..=PPU::VRAM_END => {
                self.vram[self.mirror_vram_addr(address) as usize] = value
            }
            PPU::PALETTE_RAM_START..=PPU::PALETTE_RAM_END => {
                self.palette_table[(address - PPU::PALETTE_RAM_START) as usize] = value
            }
            _ => panic!("Unexpected access to mirrored space {address:04x}"),
        };

        self.increment_ppuaddr();
    }

    pub fn write_oamdma(&mut self, value: &[u8; 256]) {
        for &x in value.iter() {
            self.oam_data[self.oamaddr.read() as usize] = x;
            self.oamaddr.inc();
        }
    }

    pub fn read_sprite_tile(&self, tile: usize) -> &[u8] {
        let bank = self.ppuctrl.sprite_pattern_address() as usize;
        &self.chr_rom[(bank + tile * 16)..=(bank + tile * 16 + 15)]
    }

    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }

    pub fn read_oamdata(&self, address: usize) -> u8 {
        self.oam_data[address]
    }

    pub fn read_palette_table(&self, address: usize) -> u8 {
        self.palette_table[address]
    }

    pub fn read_ppustatus(&mut self) -> u8 {
        let status = self.ppustatus.read();
        self.ppustatus.set(PPUSTATUS::VBLANK_FLAG, false);
        self.ppuaddr.reset_latch();
        self.ppuscroll.reset_latch();
        status
    }

    pub fn read_oamaddr(&self) -> u8 {
        self.oamaddr.read()
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let address = self.ppuaddr.read();

        self.increment_ppuaddr();

        match address {
            PPU::CHR_ROM_START..=PPU::CHR_ROM_END => {
                self.ppudata.read(self.chr_rom[address as usize])
            }
            PPU::VRAM_START..=PPU::VRAM_END => self
                .ppudata
                .read(self.vram[self.mirror_vram_addr(address) as usize]),
            PPU::PALETTE_RAM_START..=PPU::PALETTE_RAM_END => {
                self.palette_table[(address - PPU::PALETTE_RAM_START) as usize]
            }
            _ => panic!("Unexpected access to mirrored space {address:04x}"),
        }
    }

    pub fn get_x_scroll(&self) -> u8 {
        self.ppuscroll.x_scroll()
    }

    pub fn get_y_scroll(&self) -> u8 {
        self.ppuscroll.y_scroll()
    }

    pub fn read_tile(&self, tile: usize, name_table_range: &Range<usize>) -> &[u8] {
        let bank_addr = self.ppuctrl.background_pattern_address() as usize;
        let tile_index = self.vram[name_table_range.clone()][tile] as usize;
        &self.chr_rom[(bank_addr + tile_index * 16)..=(bank_addr + tile_index * 16 + 15)]
    }

    pub fn get_name_table_ranges(&self) -> (Range<usize>, Range<usize>) {
        match (&self.mirroring, self.ppuctrl.nametable_address()) {
            (Mirroring::Vertical, 0x2000)
            | (Mirroring::Vertical, 0x2800)
            | (Mirroring::Horizontal, 0x2000)
            | (Mirroring::Horizontal, 0x2400) => (0..0x400, 0x400..0x800),
            (Mirroring::Vertical, 0x2400)
            | (Mirroring::Vertical, 0x2C00)
            | (Mirroring::Horizontal, 0x2800)
            | (Mirroring::Horizontal, 0x2C00) => (0x400..0x800, 0..0x400),
            (_, _) => {
                panic!("Not supported mirroring type {:?}", self.mirroring);
            }
        }
    }

    fn increment_ppuaddr(&mut self) {
        self.ppuaddr.inc(self.ppuctrl.address_increment());
    }

    // https://www.nesdev.org/wiki/Mirroring#Nametable_Mirroring
    //
    // Horizontal Mirroring:
    //   [ A ] [ A ]
    //   [ B ] [ B ]
    //
    // Vertical Mirroring:
    //   [ A ] [ B ]
    //   [ A ] [ B ]
    fn mirror_vram_addr(&self, address: u16) -> u16 {
        let vram_index = (address & PPU::VRAM_END) - PPU::VRAM_START;
        match (&self.mirroring, vram_index / PPU::VRAM_NAMETABLE_SIZE) {
            (Mirroring::Vertical, 2 | 3) | (Mirroring::Horizontal, 3) => {
                vram_index - 2 * PPU::VRAM_NAMETABLE_SIZE
            }
            (Mirroring::Horizontal, 1 | 2) => vram_index - PPU::VRAM_NAMETABLE_SIZE,
            _ => vram_index,
        }
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool {
        let y = self.oam_data[0] as usize;
        let x = self.oam_data[3] as usize;
        (y == self.scanline as usize)
            && x <= cycle
            && self.ppumask.contains(PPUMASK::ENABLE_SPRITE_RENDERING)
    }
}
