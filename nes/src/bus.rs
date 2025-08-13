use crate::controller::controller::Controller;
use crate::ppu::ppu::PPU;
use crate::rom::rom::Rom;

pub struct Bus<'call> {
    cpu_ram: [u8; 2048],
    prg_rom: Vec<u8>,
    controller_1: Controller,
    controller_2: Controller,
    pub ppu: PPU,
    pub cycles: usize,
    nmi_callback: Box<dyn FnMut(&PPU, &mut Controller) + 'call>,
}

pub trait BusOperation<T> {
    fn read(&mut self, address: u16) -> T;

    fn write(&mut self, address: u16, value: T);
}

impl Bus<'_> {
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

    const CONTROLLER_1_ADDR: u16 = 0x4016;
    const CONTROLLER_2_ADDR: u16 = 0x4017;

    const PRG_ROM_START: u16 = 0x8000;
    const PRG_ROM_END: u16 = 0xFFFF;

    const CPU_MIRRORING: u16 = 0b0000_0111_1111_1111;
    const PPU_MIRRORING: u16 = 0b0010_0000_0000_0111;

    pub fn new<'call, F>(rom: Rom, nmi_callback: F) -> Bus<'call>
    where
        F: FnMut(&PPU, &mut Controller) + 'call,
    {
        Bus {
            cpu_ram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: PPU::new(rom.chr_rom, rom.mirroring),
            controller_1: Controller::new(),
            controller_2: Controller::new(),
            cycles: 0,
            nmi_callback: Box::new(nmi_callback),
        }
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        if self.ppu.tick(cycles * 3) {
            (self.nmi_callback)(&self.ppu, &mut self.controller_1);
        }
    }

    pub fn poll_nmi_interrupt(&mut self) -> bool {
        self.ppu.poll_nmi_interrupt()
    }
}

impl BusOperation<u8> for Bus<'_> {
    fn read(&mut self, mut address: u16) -> u8 {
        match address {
            Bus::CPU_RAM_START..=Bus::CPU_RAM_END => {
                self.cpu_ram[(address & Bus::CPU_MIRRORING) as usize]
            }
            Bus::PPUCTRL_REGISTER_ADDR
            | Bus::PPUMASK_REGISTER_ADDR
            | Bus::OAMADDR_REGISTER_ADDR
            | Bus::PPUSCROLL_REGISTER_ADDR
            | Bus::PPUADDR_REGISTER_ADDR
            | Bus::OAMDMA_REGISTER_ADDR => {
                panic!("Unable to read from writable PPU IO register - ${address:04x}")
            }
            Bus::PPUSTATUS_REGISTER_ADDR => self.ppu.read_ppustatus(),
            Bus::OAMDATA_REGISTER_ADDR => self.ppu.read_oamdata(self.ppu.read_oamaddr() as usize),
            Bus::PPUDATA_REGISTER_ADDR => self.ppu.read_ppudata(),
            Bus::PPU_IO_REGISTERS_START..=Bus::PPU_IO_REGISTERS_END => {
                self.read(address & Bus::PPU_MIRRORING)
            }
            Bus::CONTROLLER_1_ADDR => self.controller_1.read(),
            Bus::CONTROLLER_2_ADDR => self.controller_2.read(),
            Bus::PRG_ROM_START..=Bus::PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                self.prg_rom[address as usize]
            }
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            Bus::CPU_RAM_START..=Bus::CPU_RAM_END => {
                self.cpu_ram[(address & Bus::CPU_MIRRORING) as usize] = value
            }
            Bus::PPUCTRL_REGISTER_ADDR => self.ppu.write_ppuctrl(value),
            Bus::PPUMASK_REGISTER_ADDR => self.ppu.write_ppumask(value),
            Bus::OAMADDR_REGISTER_ADDR => self.ppu.write_oamaddr(value),
            Bus::OAMDATA_REGISTER_ADDR => self.ppu.write_oamdata(value),
            Bus::PPUSCROLL_REGISTER_ADDR => self.ppu.write_ppuscroll(value),
            Bus::PPUADDR_REGISTER_ADDR => self.ppu.write_ppuaddr(value),
            Bus::PPUDATA_REGISTER_ADDR => self.ppu.write_ppudata(value),
            Bus::OAMDMA_REGISTER_ADDR => {
                let hi = (value as usize) << 8;
                let buffer: [u8; 256] = (0..256)
                    .enumerate()
                    .map(|(i, _)| BusOperation::<u8>::read(self, (hi + i) as u16))
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap();
                self.ppu.write_oamdma(&buffer);
            }
            Bus::PPUSTATUS_REGISTER_ADDR => {
                panic!("Unable to write to only-readable PPU IO register - ${address:04x}")
            }
            Bus::PPU_IO_REGISTERS_START..=Bus::PPU_IO_REGISTERS_END => {
                self.write(address & Bus::PPU_MIRRORING, value)
            }
            Bus::CONTROLLER_1_ADDR => self.controller_1.write(value),
            Bus::CONTROLLER_2_ADDR => self.controller_2.write(value),
            Bus::PRG_ROM_START..=Bus::PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => {}
        }
    }
}

impl BusOperation<u16> for Bus<'_> {
    fn read(&mut self, mut address: u16) -> u16 {
        match address {
            Bus::CPU_RAM_START..=Bus::CPU_RAM_END => {
                address &= Bus::CPU_MIRRORING;
                u16::from_le_bytes([
                    self.cpu_ram[address as usize],
                    self.cpu_ram[address.wrapping_add(1) as usize],
                ])
            }
            Bus::PRG_ROM_START..=Bus::PRG_ROM_END => {
                address -= 0x8000;
                if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
                    address &= 0x3FFF;
                }
                u16::from_le_bytes([
                    self.prg_rom[address as usize],
                    self.prg_rom[address.wrapping_add(1) as usize],
                ])
            }
            _ => 0,
        }
    }

    fn write(&mut self, mut address: u16, value: u16) {
        let value_le_bytes: [u8; 2] = value.to_le_bytes();
        match address {
            Bus::CPU_RAM_START..=Bus::CPU_RAM_END => {
                address &= Bus::CPU_MIRRORING;
                self.cpu_ram[address as usize] = value_le_bytes[0];
                self.cpu_ram[address.wrapping_add(1) as usize] = value_le_bytes[1];
            }
            Bus::PRG_ROM_START..=Bus::PRG_ROM_END => panic!("Write to PRG ROM is restricted"),
            _ => {}
        }
    }
}
