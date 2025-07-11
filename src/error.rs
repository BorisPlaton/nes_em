use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("PC register contains invalid instruction {0}")]
    InvalidPC(usize),
}
