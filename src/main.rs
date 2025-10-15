mod elf64;
mod traits;
mod utils;

use elf64::Elf64Binary;
use elf64::printers::Elf64Printer;
use traits::binary_printer::BinaryPrinter;
use traits::binary::Binary;
use utils::binary_type::BinaryType;

use std::fs;
use std::io;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(help = "Path to the ELF file to be analyzed")]
    file: String,

    #[arg(short = 'e', long, help = "Displays the ELF Header of the file")]
    header: bool,

    #[arg(short = 'p', long, help = "Displays the Program Headers of the ELF file")]
    programs: bool,

    #[arg(short = 's', long, help = "Displays the Section Headers of the ELF file")]
    sections: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let bytes: Vec<u8> = fs::read(cli.file)?;
    let binary_type = BinaryType::from_bytes(&bytes[..5]);

    if binary_type.is_none() {
        eprintln!("Unrecognized binary");
        return Ok(())
    }

    match binary_type.unwrap() {
        BinaryType::Elf64 => {
            let binary = Elf64Binary::new(&bytes);

            let printer: Elf64Printer = Elf64Printer;
            if cli.header {
                printer.print_header(binary.get_header());
            } else if cli.programs {
                printer.print_program_headers(binary.get_program_headers());
            } else if cli.sections {
                printer.print_section_headers(binary.get_section_headers());
            } else {
                eprintln!("Use -h, -p or -s.");
            }
        }
    }

    Ok(())
}
