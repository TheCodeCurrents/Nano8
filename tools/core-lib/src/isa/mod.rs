use bitflags::bitflags;

pub mod opcodes;
pub mod registers;

/// Instruction Set Architecture configuration
#[derive(Debug, Clone)]
pub struct ISA {
    pub instructions: Vec<Instruction>,
    pub registers: RegisterFile,
    // Add other ISA properties
}

/// A single CPU instruction definition
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub cycles: u8,
    // Add other instruction properties
}