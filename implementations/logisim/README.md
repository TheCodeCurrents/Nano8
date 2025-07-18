# Logisim Evolution Implementation  

## Overview  
Digital logic simulation of the Nano8 CPU for pre-breadboard validation. Tests core functionality (ALU, control logic, bus structure) before physical build.  

## Status  
**Active Development** - Focused on datapath and instruction fetch/decode testing.  

## Files  
- `nano8.circ` - Main Logisim schematic
- `microcode.txt` - Control ROM definition 
- `tests/` - testing programs (e.g., ALU verification)  

## Usage  
1. Open `nano8.circ` in Logisim Evolution.  
2. Test critical paths:  
   - Clock single-step through fetch/decode/execute.  
   - Validate register/memory access.  

> **Note:** This is a **prototype**—only! Not meant to run real programs.