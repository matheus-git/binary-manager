# Binkit

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A modular toolbox for analyzing, disassembling, and patching binary formats. Currently, binkit supports ELF64 only.

**Do not use binkit to modify or inject code into third-party software without permission, as doing so may be illegal and could result in criminal or civil liability. This project is intended for educational, research, and security analysis purposes only.**

## Usage

### Info

    binkit info <FILE> --header    
    binkit info <FILE> --programs
    binkit info <FILE> --sections
    
### Inject 

Code injection is performed at the end of the binary, and then one of the program and section headers are modified to point to the injected code.
The command returns the address where the code was inserted and a reference to the original entry point or address (see the help section).
A common workflow is to update the file’s entry point with ``binkit update``, then later return to the original entry point.
The ``binkit check-inject`` command performs a pre-check of which addresses will be used so you can edit or prepare the payload before injection.
The modified section will be renamed to inject.
Use ``--help`` for more options.

    binkit inject <FILE> --inject <INJECT> --output <OUTPUT>

### Check Inject

Shows injection address and a relative reference to the return address (defaults to the entry point).

    binkit check-inject <FILE>
    binkit check-inject <FILE> --return-address <HEX_ADDRESS>

### Disassembler

Displays all instructions for the x86_64 architecture (for now) using Intel syntax. You can choose which section to disassemble.

    binkit disasm <FILE> --section <SECTION>

### Update

Update binary (currently only changes the entry point).

    binkit update <FILE> --entry <HEX_ADDRESS> 
    
## Install

### Download

Download the binary from [Releases](https://github.com/matheus-git/binkit/releases)

### Build

    cargo build --release
    ./target/release/binkit -h

### Cargo

    cargo install --locked binkit


## Updates and Contributing

This is a work-in-progress project, new features will be added over time. If you want to contribute, you can add support for other formats, such as PE or ELF32, or create an issue suggesting a feature you'd like to implement.

## 📝 License

This project is open-source under the MIT License.
