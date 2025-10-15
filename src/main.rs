mod elf64;
mod traits;
mod utils;

use elf64::Elf64Binary;
use elf64::printers::Elf64Printer;
use traits::binary_printer::BinaryPrinter;
use traits::binary_trait::Binary;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    let binary = Elf64Binary::new(&bytes);

    let printer: Elf64Printer = Elf64Printer; 
    printer.print_header(binary.get_header());
    //println!("{:?}", sections);
    Ok(())
}
