use std::fs;
use std::io;
extern crate plain;
use plain::Plain;

#[repr(C)]
#[derive(Default, Debug)]
struct ELF64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

// SAFE: ELF64Header satisfies all the requirements of `Plain`.
unsafe impl Plain for ELF64Header {}

impl ELF64Header {
	fn from_bytes(buf: &[u8]) -> &ELF64Header {
		plain::from_bytes(buf).expect("The buffer is either too short or not aligned!")
	}

	fn from_mut_bytes(buf: &mut [u8]) -> &mut ELF64Header {
		plain::from_mut_bytes(buf).expect("The buffer is either too short or not aligned!")
	}

	fn copy_from_bytes(buf: &[u8]) -> ELF64Header {
		let mut h = ELF64Header::default();
		h.copy_from_bytes(buf).expect("The buffer is too short!");
		h
	}
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
struct ELF64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

// SAFE: Satisfies `Plain` requirements
unsafe impl Plain for ELF64ProgramHeader {}

impl ELF64ProgramHeader {
    fn from_bytes(buf: &[u8]) -> &ELF64ProgramHeader {
        plain::from_bytes(buf).expect("Buffer too short or not aligned")
    }

    fn from_mut_bytes(buf: &mut [u8]) -> &mut ELF64ProgramHeader {
        plain::from_mut_bytes(buf).expect("Buffer too short or not aligned")
    }

    fn copy_from_bytes(buf: &[u8]) -> ELF64ProgramHeader {
        let mut ph = ELF64ProgramHeader::default();
        ph.copy_from_bytes(buf).expect("Buffer too short");
        ph
    }
}

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    let elf_header =  ELF64Header::from_bytes(&bytes as &[u8]);
    println!("{:?}", elf_header);

    println!("{:?}", ELF64ProgramHeader::from_bytes(&bytes[(elf_header.e_phoff as usize + elf_header.e_phentsize as usize * 4) as usize..] as &[u8]));
    Ok(())
}
