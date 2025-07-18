# Nano8

Welcome to the Nano8 General-Purpose Computer Project. The goal of this project is to balance a simple design and complex functionallity. Currently the project is still in it's planning phase so there isn't really much to see. The following is an overview of what I plan to achieve:

## Nano8 Core

The Core is an 8-bit CPU with high performance specs:
- 8 bit data and 16-bit internal addresses
- 24 bit MMU -> up to 16MiB of storage
- (hopefully) vector based interrupts
- high clock speed design (hupefully 20 MHz on the 74AC IC implementation)
- 2 stage pipeline (simple instruction prefetching mechanism)
- space efficient and elegant ISA
- Native stack operations
- Some way to handle 16-bit values (incomplete 16-bit support [not for ALU])
- Instruction decoder without ROM 

## Nano8 GPU (maybe)

There will defenitely be a VGA interface, but if I will decide to mak it more complex this will be the features:
- low resolution colored images and videos
- medium resolution 2 color images and videos
- sprite redering
- multiple supported video interfaces (VGA, SPI, I2C and maybe more)

## Nano8 Computer

The computer will hopfully be a slightly modular motherboard that contains all components and can connect to external peripherals. These are teh main specs:
- 24-bit addresses with 8/16-bit data interface (16 bit only for special read ops like instruction fetching)
- modular clock and power unit (3 clock sources, clock division and voltage from ~2.5V to 6V)
- parallel extention bus
- (possible) serial extension bus

## NanOS

The computers operating system will most likely be called NanOS or NanoOS. It will be afully fletched multitasking OS with the following features:
- multitasking
- colored terminal interface
- file system
- sufficient suit of applications (basic utils, text editor, assambler, ...)

# Implementations

There will be multiple different implementation for this computer. Maybe I won't complete all of them, but I plan the following implementations:
- Breadboard (development)
- Logisim Evolution Simulation (development)
- PCB (only for the finalized version)
- FPGA (hopefully really fast)
- Emulator (blazingly fast, written in Rust)

# Project Structure

The project structure is split into 4 sections:
- Documentation (docs/)
    - User manuals
    - Data Sheet
    - Education material
- Implementations (implementations/)
    - Folders for every single implementation
- Software (software/)
    - The OS and it's programs
    - Bare Metal Applications
    - Test programs
- Tools (tools/)
    - Core Rust library
    - CLI applications (binutils, assembler, ...)