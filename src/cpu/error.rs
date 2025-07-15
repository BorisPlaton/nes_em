use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UnknownOpCode(pub u8);

#[derive(Debug)]
pub enum StackError {
    Overflow,
    Underflow,
    OutOfStackRange(u8),
}

impl Display for UnknownOpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unknown opcode {}", self.0)
    }
}

impl Display for StackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StackError::Overflow => write!(f, "Stack overflow - exceeded 0x00 limit"),
            StackError::Underflow => write!(f, "Stack underflow - exceeded 0xFF limit"),
            StackError::OutOfStackRange(address) => write!(
                f,
                "Trying to reach out of stack memory range. Expected 0x01..0xFF, given {}",
                address
            ),
        }
    }
}

impl Error for UnknownOpCode {}
impl Error for StackError {}
