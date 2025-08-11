use crate::ppu::ppu::PPU;
use crate::rom::rom::Rom;

pub struct CPUBus<'call> {
    cpu_ram: [u8; 2048],
    prg_rom: Vec<u8>,
    pub ppu: PPU,
    pub cycles: usize,
    nmi_callback: Box<dyn FnMut(&PPU) + 'call>,
}

pub trait CPUBusOperation<T> {
    fn read(&mut self, address: u16) -> T;

    fn write(&mut self, address: u16, value: T);
}

impl CPUBus<'_> {
    const CPU_RAM_START: u16 = 0x0000;
    const CPU_RAM_END: u16 = 0x1FFF;

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

    const PRG_ROM_START: u16 = 0x8000;
    const PRG_ROM_END: u16 = 0xFFFF;

    const CPU_MIRRORING: u16 = 0b0000_0111_1111_1111;
    const PPU_MIRRORING: u16 = 0b0010_0000_0000_0111;

    pub fn new<'call, F>(rom: Rom, nmi_callback: F) -> CPUBus<'call>
    where
        F: FnMut(&PPU) + 'call,
    {
        CPUBus {
            cpu_ram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.mirroring),
            cycles: 0,
            nmi_callback: Box::new(nmi_callback),
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        if self.ppu.tick(cycles * 3) {
            (self.nmi_callback)(&self.ppu);
        }
    }

    pub fn poll_nmi_interrupt(&mut self) -> bool {
        self.ppu.poll_nmi_interrupt()
    }
}

impl CPUBusOperation<u8> for CPUBus<'_> {
    fn read(&mut self, mut address: u16) -> u8 {
        match address {
            CPUBus::CPU_RAM_START..=CPUBus::CPU_RAM_END => {
                self.cpu_ram[(address & CPUBus::CPU_MIRRORING) as usize]
            }
            CPUBus::PPUCTRL_REGISTER_ADDR
            | CPUBus::PPUMASK_REGISTER_ADDR
            | CPUBus::OAMADDR_REGISTER_ADDR
            | CPUBus::PPUSCROLL_REGISTER_ADDR
            | CPUBus::PPUADDR_REGISTER_ADDR
            | CPUBus::OAMDMA_REGISTER_ADDR => {
                panic!("Unable to read from writable PPU IO register - ${address:04x}")
            }
            CPUBus::PPUSTATUS_REGISTER_ADDR => self.ppu.read_ppustatus(),
            CPUBus::OAMDATA_REGISTER_ADDR => {
                self.ppu.read_oamdata(self.ppu.read_oamaddr() as usize)
            }
            CPUBus::PPUDATA_REGISTER_ADDR => self.ppu.read_ppudata(),
            CPUBus::PPU_IO_REGISTERS_START..=CPUBus::PPU_IO_REGISTERS_END => {
                self.read(address & CPUBus::PPU_MIRRORING)
            }
            CPUBus::PRG_ROM_START..=CPUBus::PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                self.prg_rom[address as usize]
            }
            _ => {
                // println!("Ignoring address for reading - {address:04x}");
                0
            }
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            CPUBus::CPU_RAM_START..=CPUBus::CPU_RAM_END => {
                self.cpu_ram[(address & CPUBus::CPU_MIRRORING) as usize] = value
            }
            CPUBus::PPUCTRL_REGISTER_ADDR => self.ppu.write_ppuctrl(value),
            CPUBus::PPUMASK_REGISTER_ADDR => self.ppu.write_ppumask(value),
            CPUBus::OAMADDR_REGISTER_ADDR => self.ppu.write_oamaddr(value),
            CPUBus::OAMDATA_REGISTER_ADDR => self.ppu.write_oamdata(value),
            CPUBus::PPUSCROLL_REGISTER_ADDR => self.ppu.write_ppuscroll(value),
            CPUBus::PPUADDR_REGISTER_ADDR => self.ppu.write_ppuaddr(value),
            CPUBus::PPUDATA_REGISTER_ADDR => self.ppu.write_ppudata(value),
            CPUBus::OAMDMA_REGISTER_ADDR => {
                let hi = (value as usize) << 8;
                let buffer: [u8; 256] = (0..256)
                    .enumerate()
                    .map(|(i, _)| CPUBusOperation::<u8>::read(self, (hi + i) as u16))
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                self.ppu.write_oamdma(&buffer);
            }
            CPUBus::PPUSTATUS_REGISTER_ADDR => {
                panic!("Unable to write to only-readable PPU IO register - ${address:04x}")
            }
            CPUBus::PPU_IO_REGISTERS_START..=CPUBus::PPU_IO_REGISTERS_END => {
                self.write(address & CPUBus::PPU_MIRRORING, value)
            }
            CPUBus::PRG_ROM_START..=CPUBus::PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => {
                // println!("Ignoring address for writing - {address:04x}")
            }
        }
    }
}

impl CPUBusOperation<u16> for CPUBus<'_> {
    fn read(&mut self, mut address: u16) -> u16 {
        match address {
            CPUBus::CPU_RAM_START..=CPUBus::CPU_RAM_END => {
                address &= CPUBus::CPU_MIRRORING;
                // TODO: Here probably must be an error. Reading beyond 2048
                u16::from_le_bytes([
                    self.cpu_ram[address as usize],
                    self.cpu_ram[address.wrapping_add(1) as usize],
                ])
            }
            CPUBus::PRG_ROM_START..=CPUBus::PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                // TODO: Here probably must be an error. Reading beyond 0xFFFF
                u16::from_le_bytes([
                    self.prg_rom[address as usize],
                    self.prg_rom[address.wrapping_add(1) as usize],
                ])
            }
            _ => {
                // println!("Ignoring address for reading - {address:04x}");
                0
            }
        }
    }

    fn write(&mut self, mut address: u16, value: u16) {
        let value_le_bytes: [u8; 2] = value.to_le_bytes();
        match address {
            CPUBus::CPU_RAM_START..=CPUBus::CPU_RAM_END => {
                address &= CPUBus::CPU_MIRRORING;
                self.cpu_ram[address as usize] = value_le_bytes[0];
                self.cpu_ram[address.wrapping_add(1) as usize] = value_le_bytes[1];
            }
            CPUBus::PRG_ROM_START..=CPUBus::PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => {
                println!("Ignoring address for writing - {address:04x}")
            }
        }
    }
}
