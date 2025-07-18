# Nano8 Development Tools

## Directory Structure

tools/
core-lib/          - Core shared library
nano8-asm/         - Assembler CLI
nano8-binutils/    - Binary utilities (disassembler, hexdump)
nano8-emu/         - System emulator
nano8-sim/         - Simulation tools
nano8-vasm/        - HDL generator
Cargo.toml         - Workspace configuration

## Core Components

1. core-lib
- ISA definitions
- Assembler core logic
- Binary utilities backend
- Emulator components
- Hardware interfaces

2. nano8-asm (Assembler)
Usage: cargo run --bin nano8-asm input.asm -o output.bin

3. nano8-binutils (Binary Utilities)
Commands:
- disasm: Disassembler
- hexdump: Hex viewer
- opcode: Instruction lookup
Usage: cargo run --bin nano8-binutils disasm file.bin

4. nano8-emu (Emulator)
Features:
- Cycle-accurate CPU
- Configurable memory
- VGA/SPI output
Usage: cargo run --bin nano8-emu program.bin