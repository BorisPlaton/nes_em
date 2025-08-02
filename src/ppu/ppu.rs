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

const CHR_ROM_START: u16 = 0x0000;
const CHR_ROM_END: u16 = 0x1FFF;

const PPU_VRAM_START: u16 = 0x2000;
const PPU_VRAM_END: u16 = 0x2FFF;
const PPU_VRAM_NAMETABLE_SIZE: u16 = 0x0400;

const PPU_UNUSED_SPACE_START: u16 = 0x3000;
const PPU_UNUSED_SPACE_END: u16 = 0x3EFF;

const PPU_PALETTE_RAM_START: u16 = 0x3F00;
const PPU_PALETTE_RAM_END: u16 = 0x3FFF;

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

    // Internal Registers
    // https://www.nesdev.org/wiki/PPU_registers#Internal_registers
    register_w: bool,

    chr_rom: Vec<u8>,
    mirroring: Mirroring,
    vram: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],
}

impl PPU {
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

            register_w: true,

            chr_rom,
            mirroring,
            vram: [0; 2048],
            palette_table: [0; 32],
            oam_data: [0; 256],
        }
    }

    pub fn write_ppuctrl(&mut self, value: u8) {
        self.ppuctrl.write(value);
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
        self.ppuscroll.write(value, &mut self.register_w);
    }

    pub fn write_ppuaddr(&mut self, address_part: u8) {
        self.ppuaddr.write(address_part, &mut self.register_w);
    }

    pub fn write_ppudata(&mut self, value: u8) {
        let address = self.ppuaddr.read();
        self.increment_ppuaddr();

        match address {
            CHR_ROM_START..=CHR_ROM_END => self.chr_rom[address as usize] = value,
            PPU_VRAM_START..=PPU_VRAM_END | PPU_UNUSED_SPACE_START..=PPU_UNUSED_SPACE_END => {
                self.vram[self.mirror_vram_addr(address) as usize] = value
            }
            PPU_PALETTE_RAM_START..=PPU_PALETTE_RAM_END => {
                self.palette_table[(address - PPU_PALETTE_RAM_START) as usize] = value
            }
            _ => panic!("unexpected access to mirrored space {address}"),
        };
    }

    pub fn write_oamdma(&mut self, value: u8) {
        self.oamdma.write(value);
    }

    pub fn read_ppustatus(&mut self) -> u8 {
        self.ppustatus.read(&mut self.register_w)
    }

    pub fn read_oamdata(&self) -> u8 {
        self.oam_data[self.oamaddr.read() as usize]
    }

    pub fn read_ppudata(&mut self) -> u8 {
        let address = self.ppuaddr.read();
        self.increment_ppuaddr();

        match address {
            CHR_ROM_START..=CHR_ROM_END => self.ppudata.read(self.chr_rom[address as usize]),
            PPU_VRAM_START..=PPU_VRAM_END | PPU_UNUSED_SPACE_START..=PPU_UNUSED_SPACE_END => self
                .ppudata
                .read(self.vram[self.mirror_vram_addr(address) as usize]),
            PPU_PALETTE_RAM_START..=PPU_PALETTE_RAM_END => {
                self.palette_table[(address - PPU_PALETTE_RAM_START) as usize]
            }
            _ => panic!("unexpected access to mirrored space {address}"),
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
        let vram_index = (address & PPU_VRAM_END) - PPU_VRAM_START;
        let name_table_number = vram_index / PPU_VRAM_NAMETABLE_SIZE;
        match (&self.mirroring, name_table_number) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) | (Mirroring::Horizontal, 3) => {
                vram_index - 2 * PPU_VRAM_NAMETABLE_SIZE
            }
            (Mirroring::Horizontal, 1) | (Mirroring::Horizontal, 2) => {
                vram_index - PPU_VRAM_NAMETABLE_SIZE
            }
            _ => vram_index,
        }
    }
}
