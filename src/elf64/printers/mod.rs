use crate::traits::binary_printer::BinaryPrinter;
use crate::traits::header_field::HeaderField;

use super::types::{elf64_header::Elf64Header, elf64_program_header::Elf64ProgramHeader, elf64_section_header::Elf64SectionHeader};
use tabled::{Table, Tabled};
use tabled::settings::{Settings, Style, Padding};
use tabled::builder::Builder;

pub struct Elf64Printer;

impl BinaryPrinter for Elf64Printer {
    type Header = Elf64Header;
    type ProgramHeader = Elf64ProgramHeader;
    type SectionHeader = Elf64SectionHeader;

    fn print_header(&self, header: &Self::Header){
        #[derive(Tabled)]
        struct HeaderField<'a> {
            name: &'a str,
            value: &'a str,
            describe: String,
        }

        let fields = vec![
            HeaderField { name: "e_ident", value: &header.e_ident.as_hex, describe: header.e_ident.describe() },
            HeaderField { name: "e_type", value: &header.e_type.as_hex, describe: header.e_type.describe() },
            HeaderField { name: "e_machine", value: &header.e_machine.as_hex, describe: header.e_machine.describe() },
            HeaderField { name: "e_version", value: &header.e_version.as_hex, describe: header.e_version.describe() },
            HeaderField { name: "e_entry", value: &header.e_entry.as_hex, describe: header.e_entry.describe() },
            HeaderField { name: "e_phoff", value: &header.e_phoff.as_hex, describe: header.e_phoff.describe() },
            HeaderField { name: "e_shoff", value: &header.e_shoff.as_hex, describe: header.e_shoff.describe() },
            HeaderField { name: "e_flags", value: &header.e_flags.as_hex, describe: header.e_flags.describe() },
            HeaderField { name: "e_ehsize", value: &header.e_ehsize.as_hex, describe: header.e_ehsize.describe() },
            HeaderField { name: "e_phentsize", value: &header.e_phentsize.as_hex, describe: header.e_phentsize.describe() },
            HeaderField { name: "e_phnum", value: &header.e_phnum.as_hex, describe: header.e_phnum.describe() },
            HeaderField { name: "e_shentsize", value: &header.e_shentsize.as_hex, describe: header.e_shentsize.describe() },
            HeaderField { name: "e_shnum", value: &header.e_shnum.as_hex, describe: header.e_shnum.describe() },
            HeaderField { name: "e_shstrndx", value: &header.e_shstrndx.as_hex, describe: header.e_shstrndx.describe() },
        ];

        let table_config = Settings::default()
            .with(Style::ascii());
        let table = Table::new(fields).with(table_config).to_string();
        println!("{}", table);
    }

    fn print_program_headers(&self, phs: &[Self::ProgramHeader]) {
        #[derive(Tabled)]
        struct ProgramHeaderFields {
            p_type: String,
            p_offset: String,
            p_vaddr: String,
            p_paddr: String,
            p_filesz: String,
            p_memsz: String,
            p_flags: String,
            p_align: String,
        }

        let table_config = Settings::default()
            .with(Style::ascii());

        let mut fields: Vec<ProgramHeaderFields> = Vec::with_capacity(phs.len());

        for ph in phs {
            fields.push(
                ProgramHeaderFields { 
                    p_type: ph.p_type.describe(),
                    p_offset: ph.p_offset.describe(),
                    p_vaddr: ph.p_vaddr.describe(),
                    p_paddr: ph.p_paddr.describe(),
                    p_filesz: ph.p_filesz.describe(),
                    p_memsz: ph.p_memsz.describe(),
                    p_flags: ph.p_flags.describe(),
                    p_align: ph.p_align.describe()
                }
            );
        }

        let table = Table::new(fields).with(table_config).to_string();

        println!("{}", table);
    }

    fn print_section_headers(&self, shs: &[Self::SectionHeader]) {
        #[derive(Tabled)]
        struct SectionHeaderFields {
            sh_name: String,
            sh_type: String,
            sh_flags: String,
            sh_addr: String,
            sh_offset: String,
            sh_size: String,
            sh_link: String,
            sh_info: String,
            sh_addralign: String,
            sh_entsize: String,
        }

        let table_config = Settings::default()
            .with(Style::ascii());

        let mut fields: Vec<SectionHeaderFields> = Vec::with_capacity(shs.len());

        for sh in shs {
            fields.push(
                SectionHeaderFields { 
                    sh_name: sh.sh_name.describe(),
                    sh_type: sh.sh_type.describe(),
                    sh_flags: sh.sh_flags.describe(),
                    sh_addr: sh.sh_addr.describe(),
                    sh_offset: sh.sh_offset.describe(),
                    sh_size: sh.sh_size.describe(),
                    sh_link: sh.sh_link.describe(),
                    sh_info: sh.sh_info.describe(),
                    sh_addralign: sh.sh_addralign.describe(),
                    sh_entsize: sh.sh_entsize.describe()
                }
            );
        }

        let table = Table::new(fields).with(table_config).to_string();

        println!("{}", table);
    }
}
