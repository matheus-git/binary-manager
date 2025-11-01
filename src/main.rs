mod elf64;
mod traits;
mod utils;
mod disasm;

use elf64::Elf64Binary;
use elf64::printers::Elf64Printer;
use traits::binary_printer::BinaryPrinter;
use traits::binary::Binary;
use disasm::disass;
use utils::parse_hex::parse_hex;

use std::fs;
use std::os::unix::fs::PermissionsExt;

use clap::{Parser, Error, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Inject bytes into an ELF file")]
    Inject {
        #[arg(help = "Path to the ELF file to analyze")]
        file: String,

        #[arg(short = 'i', long, help = "Path to the file containing bytes to inject")]
        inject: String,

        #[arg(short = 'a', long, help = "Path to the file containing bytes to inject")]
        address: Option<String>,

        #[arg(short = 's', long, help = "Path to the file containing bytes to inject")]
        section: Option<String>,

        #[arg(short = 'r', long, help = "Path to the file containing bytes to inject")]
        return_address: Option<String>,

        #[arg(short = 'o', long, help = "Path to save the output file")]
        output: String,
    },

    CheckInject {
        #[arg(help = "Path to the ELF file to analyze")]
        file: String,

        #[arg(short = 'r', long, help = "Path to the file containing bytes to inject")]
        return_address: Option<String>,
    },

    #[command(about = "Disassemble a section or address range of an ELF file")]
    Disasm {
       #[arg(help = "Path to the ELF file to analyze")]
        file: String,

       #[arg(short = 's', long = "section", help = "Section name to disassemble (e.g. .text)")]
        section: Option<String>,
    },
    #[command(about = "Display ELF information")]
    Info {
        #[arg(help = "Path to the ELF file to analyze")]
        file: String,

        #[arg(short = 'e', long, help = "Display the ELF header of the file")]
        header: bool,

        #[arg(short = 'p', long, help = "Display the program headers of the ELF file")]
        programs: bool,

        #[arg(short = 's', long, help = "Display the section headers of the ELF file")]
        sections: bool,
    },
    #[command(about = "Update ELF metadata (e.g., entry point)")]
    Update {
        #[arg(help = "Path to the ELF file to analyze")]
        file: String,       

        #[arg(long, help = "Set entry point (e.g. 0x401000)")]
        entry: Option<String>,

        #[arg(short = 'o', long, help = "Path to save the output file")]
        output: String,
    }
}

fn load_file(file: &str) -> Result<Elf64Binary, Error> {
    let bytes: Vec<u8> = fs::read(file)?;

    Ok(Elf64Binary::new(&bytes))
}

fn save_file(file: &str, buf: &[u8]) -> Result<(), Error>{
    let _ = fs::write(file, buf);
    let mut perms = fs::metadata(&file)?.permissions();
    perms.set_mode(0o755); 
    fs::set_permissions(&file, perms)?;
    Ok(())
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut binary: Elf64Binary;

    let printer: Elf64Printer = Elf64Printer;

    match &cli.command {
        Commands::Inject { file, address, return_address, inject, output, section } => {
            binary = load_file(file)?;

            let bytes = fs::read(inject)?; 

            let mut final_address = binary.get_address_to_inject();
            let mut final_return_address = binary.entry();
            let mut final_section = ".note.ABI-tag";

            if let Some(address) = address.as_ref() {
                final_address = parse_hex(address);
            }

            if let Some(return_address) = return_address.as_ref() {
                final_return_address = parse_hex(return_address);
            }

            if let Some(section) = section.as_ref() {
                final_section = section;
            }

            let injected: Vec<u8> = binary.inject(bytes, final_address, final_section);
            println!("Payload injected at 0x{:X}", final_address);
            let rel32_addr = binary.calculate_rel32(final_address, final_return_address);
            if return_address.is_some() {
                println!("Rel32 to 0x{:X}: 0x{:X}", final_return_address, rel32_addr);
            }else {
                println!("Rel32 to original entry point (0x{:X}): 0x{:X}", final_return_address, rel32_addr);
            }
            save_file(output, &injected)?;
            println!("Output written to: {}", output);
        },
        Commands::CheckInject { file, return_address } => {
            binary = load_file(file)?;

            let mut final_return_address = binary.entry();

            if let Some(return_address) = return_address.as_ref() {
                final_return_address = parse_hex(return_address);
            }

            let addr = binary.get_address_to_inject();
            println!("Injection slot available at: 0x{:X}", addr);

            let rel32_addr = binary.calculate_rel32(addr, final_return_address);
            if return_address.is_some() {
                println!("Rel32 relative to 0x{:X}: 0x{:X}", final_return_address, rel32_addr);
            }else {
                println!("Rel32 to original entry point (0x{:X}): 0x{:X}", final_return_address, rel32_addr);
            }
        },
        Commands::Disasm { file, section } => {
            binary = load_file(file)?;

            if let Some(section) = section.as_ref() {
                if let Some((addr, bytes)) = binary.get_bytes_section(section) {
                    disass(addr, &bytes);
                }
            }
        },
        Commands::Info { file, header, programs, sections } => {
            binary = load_file(file)?;
            if *header {
                printer.print_header(binary.get_header());
            } else if *programs {
                printer.print_program_headers(binary.get_program_headers());
            } else if *sections {
                printer.print_section_headers(binary.get_section_headers());
            } 
        },
        Commands::Update { file, entry, output } => {
            binary = load_file(file)?;

            if entry.is_some() {
                binary.set_entry(entry.as_ref().unwrap().to_string());
                let bytes: Vec<u8> = (&binary).into();
                save_file(output, &bytes)?;
                println!("Generated at {output}");
            }
        }
    }

    Ok(())
}
