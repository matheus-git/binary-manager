mod elf64;
mod traits;
mod utils;

use elf64::Elf64Binary;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    let binary = Elf64Binary::new(&bytes);
    println!("{:?}", binary);
    Ok(())
}
