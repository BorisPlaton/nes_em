use crate::ppu::ppu::PPU;
use crate::rom::rom::Rom;

const CPU_RAM_START: u16 = 0x0000;
const CPU_RAM_END: u16 = 0x1FFF;
const CPU_MIRRORING: u16 = 0b0000_0111_1111_1111;

const PPUCTRL_REGISTER_ADDR: u16 = 0x2000;
const PPUMASK_REGISTER_ADDR: u16 = 0x2001;
const PPUSTATUS_REGISTER_ADDR: u16 = 0x2002;
const OAMADDR_REGISTER_ADDR: u16 = 0x2003;
const OAMDATA_REGISTER_ADDR: u16 = 0x2004;
const PPUSCROLL_REGISTER_ADDR: u16 = 0x2005;
const PPUADDR_REGISTER_ADDR: u16 = 0x2006;
const PPUDATA_REGISTER_ADDR: u16 = 0x2007;
const OAMDMA_REGISTER_ADDR: u16 = 0x4014;
const PPU_IO_REGISTERS_START: u16 = 0x2008;
const PPU_IO_REGISTERS_END: u16 = 0x3FFF;
const PPU_MIRRORING: u16 = 0b0010_0000_0000_0111;

const PRG_ROM_START: u16 = 0x8000;
const PRG_ROM_END: u16 = 0xFFFF;

pub struct CPUBus {
    cpu_ram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,
    cycles: usize,
}

pub trait IOOperation<T> {
    fn read(&mut self, address: u16) -> T;

    fn write(&mut self, address: u16, value: T);
}

impl CPUBus {
    pub fn new(rom: Rom) -> CPUBus {
        CPUBus {
            cpu_ram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.mirroring),
            cycles: 0,
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        self.ppu.tick(cycles * 3);
    }
}

impl IOOperation<u8> for CPUBus {
    fn read(&mut self, mut address: u16) -> u8 {
        match address {
            CPU_RAM_START..=CPU_RAM_END => self.cpu_ram[(address & CPU_MIRRORING) as usize],
            PPUCTRL_REGISTER_ADDR
            | PPUMASK_REGISTER_ADDR
            | OAMADDR_REGISTER_ADDR
            | PPUSCROLL_REGISTER_ADDR
            | PPUADDR_REGISTER_ADDR
            | OAMDMA_REGISTER_ADDR => {
                panic!("Unable to read from writable PPU IO register - ${address:04x}")
            }
            PPUSTATUS_REGISTER_ADDR => self.ppu.read_ppustatus(),
            OAMDATA_REGISTER_ADDR => self.ppu.read_oamdata(),
            PPUDATA_REGISTER_ADDR => self.ppu.read_ppudata(),
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => self.read(address & PPU_MIRRORING),
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
            CPU_RAM_START..=CPU_RAM_END => self.cpu_ram[(address & CPU_MIRRORING) as usize] = value,
            PPUCTRL_REGISTER_ADDR => self.ppu.write_ppuctrl(value),
            PPUMASK_REGISTER_ADDR => self.ppu.write_ppumask(value),
            OAMADDR_REGISTER_ADDR => self.ppu.write_oamaddr(value),
            OAMDATA_REGISTER_ADDR => self.ppu.write_oamdata(value),
            PPUSCROLL_REGISTER_ADDR => self.ppu.write_ppuscroll(value),
            PPUADDR_REGISTER_ADDR => self.ppu.write_ppuaddr(value),
            PPUDATA_REGISTER_ADDR => self.ppu.write_ppudata(value),
            OAMDMA_REGISTER_ADDR => self.ppu.write_oamdma(value),
            PPUSTATUS_REGISTER_ADDR => {
                panic!("Unable to write to only-readable PPU IO register - ${address:04x}")
            }
            PPU_IO_REGISTERS_START..=PPU_IO_REGISTERS_END => {
                self.write(address & PPU_MIRRORING, value)
            }
            PRG_ROM_START..=PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => panic!("Invalid address"),
        }
    }
}

impl IOOperation<u16> for CPUBus {
    fn read(&mut self, mut address: u16) -> u16 {
        match address {
            CPU_RAM_START..=CPU_RAM_END => {
                address &= CPU_MIRRORING;
                // TODO: Here probably must be an error. Reading beyond 2048
                u16::from_le_bytes([
                    self.cpu_ram[address as usize],
                    self.cpu_ram[(address + 1) as usize],
                ])
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
                address &= CPU_MIRRORING;
                self.cpu_ram[address as usize] = value_le_bytes[0];
                self.cpu_ram[(address + 1) as usize] = value_le_bytes[1];
            }
            PRG_ROM_START..=PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => panic!("Invalid address"),
        }
    }
}
