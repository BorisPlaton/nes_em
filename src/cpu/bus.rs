use crate::ppu::ppu::PPU;
use crate::rom::rom::Rom;

const CPU_RAM_START: u16 = 0x0000;
const CPU_RAM_END: u16 = 0x1FFF;

const PPU_IO_REGISTERS_START: u16 = 0x2000;
const PPU_IO_REGISTERS_END: u16 = 0x3FFF;

const PRG_ROM_START: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub struct CPUBus {
    cpu_ram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
}

pub trait IOOperation<T> {
    fn read(&self, address: u16) -> T;

    fn write(&mut self, address: u16, value: T);
}

impl CPUBus {
    pub fn new(rom: Rom) -> CPUBus {
        CPUBus {
            cpu_ram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.mirroring),
        }
    }
}

impl IOOperation<u8> for CPUBus {
    fn read(&self, mut address: u16) -> u8 {
        match address {
            CPU_RAM_START..=CPU_RAM_END => self.cpu_ram[(address & 0b0000_0111_1111_1111) as usize],
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => {
                // address & 0b0010_0000_0000_0111;
                panic!("Not implemented for PPU IO registers")
            }
            PRG_ROM_START..=PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                self.prg_rom[address as usize]
            }
            _ => panic!("Invalid address"),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            CPU_RAM_START..=CPU_RAM_END => {
                self.cpu_ram[(address & 0b0000_0111_1111_1111) as usize] = value
            }
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => {
                // address & 0b0010_0000_0000_0111;
                panic!("Not implemented for PPU IO registers")
            }
            PRG_ROM_START..=PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => panic!("Invalid address"),
        }
    }
}

impl IOOperation<u16> for CPUBus {
    fn read(&self, mut address: u16) -> u16 {
        match address {
            CPU_RAM_START..=CPU_RAM_END => {
                address &= 0b0000_0111_1111_1111;
                // TODO: Here probably must be an error. Reading beyond 2048
                u16::from_le_bytes([
                    self.cpu_ram[address as usize],
                    self.cpu_ram[(address + 1) as usize],
                ])
            }
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => {
                // address & 0b0010_0000_0000_0111;
                panic!("Not implemented for PPU IO registers")
            }
            PRG_ROM_START..=PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                // TODO: Here probably must be an error. Reading beyond 0xFFFF
                u16::from_le_bytes([
                    self.prg_rom[address as usize],
                    self.prg_rom[(address + 1) as usize],
                ])
            }
            _ => panic!("Invalid address"),
        }
    }

    fn write(&mut self, mut address: u16, value: u16) {
        let value_le_bytes: [u8; 2] = value.to_le_bytes();
        match address {
            CPU_RAM_START..=CPU_RAM_END => {
                address &= 0b0000_0111_1111_1111;
                self.cpu_ram[address as usize] = value_le_bytes[0];
                self.cpu_ram[(address + 1) as usize] = value_le_bytes[1];
            }
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => {
                // address & 0b0010_0000_0000_0111;
                panic!("Not implemented for PPU IO registers")
            }
            PRG_ROM_START..=PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => panic!("Invalid address"),
        }
    }
}
