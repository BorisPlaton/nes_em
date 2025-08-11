use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum InvalidINESFile<'a> {
    IncorrectNESTag(&'a [u8], [u8; 4]),
    PRGROMSizeAbsent,
    CHRROMSizeAbsent,
    ControlByte1Absent,
    ControlByte2Absent,
    FailedToReadPRGROM,
    FailedToReadCHRROM,
}

impl Display for InvalidINESFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidINESFile::IncorrectNESTag(actual, expected) => {
                write!(
                    f,
                    "First 4 bytes of iNES1.0 expects to be {:?}, actual {:?}",
                    expected, actual
                )
            }
            InvalidINESFile::PRGROMSizeAbsent => write!(f, "4 bytes doesn't contain a PRGROM size"),
            InvalidINESFile::CHRROMSizeAbsent => write!(f, "5 bytes doesn't contain a CHRROM size"),
            InvalidINESFile::ControlByte1Absent => {
                write!(f, "6 bytes doesn't contain a control byte 1")
            }
            InvalidINESFile::ControlByte2Absent => {
                write!(f, "7 bytes doesn't contain a control byte 2")
            }
            InvalidINESFile::FailedToReadPRGROM => write!(f, "Failed to read PRGROM data"),
            InvalidINESFile::FailedToReadCHRROM => write!(f, "Failed to read CHRROM data"),
        }
    }
}

impl Error for InvalidINESFile<'_> {}
