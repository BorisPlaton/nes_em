use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct InvalidBankNumber(pub u8);

impl Display for InvalidBankNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid bank number - {}", self.0)
    }
}

impl Error for InvalidBankNumber {}
