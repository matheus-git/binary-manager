# Binary Manager

![rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)

A binary parser that currently supports only ELF64 files. It displays the binary's headers in a structured table format. Once the binary is parsed, additional features can be added over time, such as code injection and other analysis tools.

The -h flag is available for any usability questions.

## Example

![screenshot_list](https://raw.githubusercontent.com/matheus-git/systemd-manager-tui/main/assets/systemd-manager-tui.gif)

## Install

### Build

    cargo build --release
    ./target/release/binary-manager -h

### Cargo

    cargo install --locked binary-manager

## Updates and Contributing

This is a work-in-progress project; new features will be added over time. If you want to contribute, you can add support for other formats, such as PE or ELF32, or create an issue suggesting a feature you'd like to implement.

## 📝 License

This project is open-source under the MIT License.
