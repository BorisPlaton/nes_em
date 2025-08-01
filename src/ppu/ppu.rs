use crate::ppu::mirroring::Mirroring;
use crate::ppu::register::addr::AddressRegister;
use crate::ppu::register::control::ControlRegister;
use crate::ppu::register::data::DataRegister;

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
    address_register: AddressRegister,
    data_register: DataRegister,
    control_register: ControlRegister,

    chr_rom: Vec<u8>,
    mirroring: Mirroring,
    vram: [u8; 2048],
    palette_table: [u8; 32],
    oam_data: [u8; 256],
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        PPU {
            address_register: AddressRegister::new(),
            data_register: DataRegister::new(),
            control_register: ControlRegister::new(),

            chr_rom,
            mirroring,
            vram: [0; 2048],
            palette_table: [0; 32],
            oam_data: [0; 256],
        }
    }

    pub fn write_ppuaddr(&mut self, address_part: u8) {
        self.address_register.update(address_part);
    }

    pub fn write_to_ppuctrl(&mut self, value: u8) {
        self.control_register.update(value);
    }

    pub fn read_from_ppuaddr(&mut self) -> u8 {
        let address = self.address_register.get();
        self.increment_ppuaddr();

        match address {
            CHR_ROM_START..=CHR_ROM_END => self.data_register.read(self.chr_rom[address as usize]),
            PPU_VRAM_START..=PPU_VRAM_END | PPU_UNUSED_SPACE_START..=PPU_UNUSED_SPACE_END => self
                .data_register
                .read(self.vram[self.mirror_vram_addr(address) as usize]),
            PPU_PALETTE_RAM_START..=PPU_PALETTE_RAM_END => {
                self.palette_table[(address - PPU_PALETTE_RAM_START) as usize]
            }
            _ => panic!("unexpected access to mirrored space {address}"),
        }
    }

    pub fn write_to_ppuadr(&mut self) {
        let address = self.address_register.get();
        let value = self.data_register.get_write_value();
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

    fn increment_ppuaddr(&mut self) {
        self.address_register
            .increment(self.control_register.address_increment());
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
