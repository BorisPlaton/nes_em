use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UnknownOpCode(pub u8);

impl Display for UnknownOpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown opcode {}", self.0)
    }
}

impl Error for UnknownOpCode {}
