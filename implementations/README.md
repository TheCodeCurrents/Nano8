# Implementations

This directory contains various implementations of the Nano8 computer system. Each implementation represents a different approach to realizing the Nano8 architecture.

## Available Implementations

### 1. Breadboard (Development)
- **Location**: `breadboard/`
- **Status**: Development
- **Description**: Physical prototype using 74-series logic chips on breadboards. Primary development platform for testing the Nano8 architecture.

### 2. Logisim Evolution Simulation
- **Location**: `logisim/`
- **Status**: Development
- **Description**: Digital logic simulation using Logisim Evolution for design validation before physical implementation.

### 3. PCB (Final Hardware)
- **Location**: `pcb/`
- **Status**: Planned
- **Description**: Final printed circuit board implementation to be developed after architecture finalization.

### 4. FPGA Implementation
- **Location**: `fpga/`
- **Status**: Planned
- **Description**: High-performance FPGA implementation of the Nano8 architecture.

### 5. Software Emulator
- **Location**: `emulator/`
- **Status**: Planned
- **Description**: Rust-based emulator for software development and testing.

## Implementation Structure

Each implementation directory contains:
- Implementation-specific documentation
- Design files (schematics, HDL, etc.)
- Build/usage instructions
- Any necessary tools or utilities

> Note: FPGA and Logisim implementations are currently the most active development targets. Breadboarding will started when I recieved some components. The Rust based Emulator will be done as a side effort and the PCB will be the finalized product.