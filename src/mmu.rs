use goblin::{error::Result, Object};
use std::fs;
use std::path::Path;

use crate::processor::State;

impl State {
    pub fn read_elf(&mut self) -> Result<()> {
        let path = Path::new("./test/test.out");
        let buffer = fs::read(path)?;
        match Object::parse(&buffer)? {
            Object::Elf(elf) => {
                // println!("elf: {:#?}", &elf);
                let shdr_strtab = &elf.shdr_strtab;
                for section in &elf.section_headers {
                    println!(
                    "elf.section_headers = {:#?}, file_offset = {:#x}, size = {:#x}, addr = {:#x}",
                    &shdr_strtab[section.sh_name],
                    section.sh_offset,
                    section.sh_size,
                    section.sh_addr
                );
                    for idx in 0..section.sh_size {
                        let offset = idx + section.sh_offset;
                        self.memory
                            .insert(section.sh_addr + idx, buffer[offset as usize]);
                    }
                }

                self.pc = elf.header.e_entry as i64;
            }
            Object::PE(pe) => {
                println!("pe: {:#?}", &pe);
            }
            Object::Mach(mach) => {
                println!("mach: {:#?}", &mach);
            }
            Object::Archive(archive) => {
                println!("archive: {:#?}", &archive);
            }
            Object::Unknown(magic) => {
                println!("unknown magic: {:#x}", magic)
            }
        }
        Ok(())
    }
}
