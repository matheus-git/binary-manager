mod elf64;
mod traits;
mod utils;
mod disasm;

use elf64::Elf64Binary;
use elf64::printers::Elf64Printer;
use traits::binary_printer::BinaryPrinter;
use traits::binary::Binary;
use disasm::disass;

use std::fs;
use std::os::unix::fs::PermissionsExt;

use clap::{Parser, Error,  Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Inject {
        #[arg(help = "Path to the ELF file to be analyzed")]
        file: String,

        #[arg(short = 'i', long, help = "Path to inject file")]
        inject: String,

        #[arg(short = 'o', long, help = "Path to save the output file")]
        output: String,
    },
    Disasm {
       #[arg(help = "Path to the ELF file to be analyzed")]
        file: String,

       #[arg(short = 's', long = "section", help = "Disassemble file")]
        section: Option<String>,
    },
    Info {
        #[arg(help = "Path to the ELF file to be analyzed")]
        file: String,

        #[arg(short = 'e', long, help = "Displays the ELF Header of the file")]
        header: bool,

        #[arg(short = 'p', long, help = "Displays the Program Headers of the ELF file")]
        programs: bool,

        #[arg(short = 's', long, help = "Displays the Section Headers of the ELF file")]
        sections: bool,
    },
    Update {
        #[arg(help = "Path to the ELF file to be analyzed")]
        file: String,       

        #[arg(long, help = "Set entry")]
        entry: Option<String>,

        #[arg(short = 'o', long, help = "Path to save the output file")]
        output: String,
    }
}

fn load_file(file: &str) -> Elf64Binary {
    let bytes: Vec<u8> = fs::read(file).expect("failed");

    Elf64Binary::new(&bytes)
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    let mut binary: Elf64Binary;

    let printer: Elf64Printer = Elf64Printer;

    match &cli.command {
        Commands::Inject { file, inject, output } => {
            binary = load_file(file);

            let bytes = fs::read(inject)?; 

            let injected: Vec<u8> = binary.inject(bytes);
            let _ = fs::write(&output, injected);
            let mut perms = fs::metadata(&output)?.permissions();
            perms.set_mode(0o755); 
            fs::set_permissions(&output, perms)?;
            println!("Generated at {output}");
        },
        Commands::Disasm { file, section } => {
            binary = load_file(file);

            let section =  section
                .as_ref();
            
            let mut bytes: Vec<u8> = (&binary).into();
            let mut addr: u64 = 0;

            if let Some(section) = section {
                if let Some((_addr, _bytes)) = binary.get_bytes_section(section) {
                    addr = _addr;
                    bytes = _bytes;
                }
            }

            disass(addr, &bytes);
        },
        Commands::Info { file, header, programs, sections } => {
            binary = load_file(file);
            if *header {
                printer.print_header(binary.get_header());
            } else if *programs {
                printer.print_program_headers(binary.get_program_headers());
            } else if *sections {
                printer.print_section_headers(binary.get_section_headers());
            } 
        },
        Commands::Update { file, entry, output } => {
            binary = load_file(file);

            if entry.is_some() {
                binary.set_entry(entry.as_ref().unwrap().to_string());
                let bytes: Vec<u8> = (&binary).into();
                let _ = fs::write(&output, bytes);
                let mut perms = fs::metadata(&output)?.permissions();
                perms.set_mode(0o755); 
                fs::set_permissions(&output, perms)?;
                println!("Generated at {output}");
            }
        }
    }


    Ok(())
}
