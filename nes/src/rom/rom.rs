use crate::ppu::mirroring::Mirroring;
use crate::rom::control_bytes::{ControlBytes, NESFormat};
use crate::rom::error::InvalidINESFile;

pub const NES_TAG: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
const PRG_ROM_SIZE: usize = 16384;
const CHRROM_SIZE: usize = 8192;

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    mapper: u8,
    pub mirroring: Mirroring,
}

impl Rom {
    pub fn new(content: &[u8]) -> Result<Self, InvalidINESFile> {
        let nes_tag = content
            .get(0..4)
            .ok_or(InvalidINESFile::IncorrectNESTag(&[]))?;
        if nes_tag != NES_TAG {
            return Err(InvalidINESFile::IncorrectNESTag(nes_tag));
        }

        let prg_rom_size =
            *content.get(4).ok_or(InvalidINESFile::PRGROMSizeAbsent)? as usize * PRG_ROM_SIZE;
        let chr_rom_size =
            *content.get(5).ok_or(InvalidINESFile::CHRROMSizeAbsent)? as usize * CHRROM_SIZE;
        let control_bytes = ControlBytes::new(
            *content.get(6).ok_or(InvalidINESFile::ControlByte1Absent)?,
            *content.get(7).ok_or(InvalidINESFile::ControlByte2Absent)?,
        );

        if control_bytes.nes_format() == NESFormat::NES2 {
            panic!("NES2.0 isn't supported")
        }

        let prg_rom_start = 16 + control_bytes.trainer_size();
        let chr_rom_start = prg_rom_start + prg_rom_size;

        Ok(Rom {
            prg_rom: content
                .get(prg_rom_start..(prg_rom_start + prg_rom_size))
                .ok_or(InvalidINESFile::FailedToReadPRGROM)?
                .try_into()
                .unwrap(),
            chr_rom: content
                .get(chr_rom_start..(chr_rom_start + chr_rom_size))
                .ok_or(InvalidINESFile::FailedToReadCHRROM)?
                .try_into()
                .unwrap(),
            mapper: control_bytes.mapper(),
            mirroring: control_bytes.mirroring(),
        })
    }
}
