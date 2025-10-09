mod elf64;
mod traits;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    elf64::load(bytes);
    Ok(())
}
