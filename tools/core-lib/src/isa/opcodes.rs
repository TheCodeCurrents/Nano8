use super::ISA;

/// All supported opcodes
pub const OP_LDA: u8 = 0x01;
pub const OP_STA: u8 = 0x02;
// Add other opcodes

/// Initialize ISA with all instructions
pub fn create_isa() -> ISA {
    ISA {
        instructions: vec![
            Instruction {
                opcode: OP_LDA,
                mnemonic: "LDA",
                cycles: 2,
            },
            // Add other instructions
        ],
        // Initialize other ISA fields
    }
}