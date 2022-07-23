use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum ErrorType {
    InvalidAddr,
    InvalidBitAddr,
    InstructionArg0Missing,
    InstructionArg1Missing,
    UnimplementedInstruction,
    UninitializedCPU,
    UnknownInstruction,
}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorType::InvalidAddr => "Invalid address",
            ErrorType::InvalidBitAddr => "Invalid bit address",
            ErrorType::InstructionArg0Missing => "Instruction argument 0 missing",
            ErrorType::InstructionArg1Missing => "Instruction argument 1 missing",
            ErrorType::UnimplementedInstruction => "Unimplemented instruction",
            ErrorType::UninitializedCPU => "Uninitialized CPU",
            ErrorType::UnknownInstruction => "Unknown instruction",
        }
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    error_type: ErrorType,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.error_type)
    }
}

impl std::error::Error for RuntimeError {}

impl RuntimeError {
    pub fn new(error_type: ErrorType) -> Self {
        RuntimeError {
            error_type,
        }
    }
}
