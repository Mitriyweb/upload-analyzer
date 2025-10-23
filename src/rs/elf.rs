use goblin::elf::Elf;
use std::collections::HashMap;

pub fn get_file_info(data: &[u8]) -> HashMap<String, String> {
    let mut info = HashMap::new();
    info.insert("type".to_string(), "ELF (Linux/Unix)".to_string());
    info.insert("size".to_string(), data.len().to_string());
    info
}

pub fn parse_elf_metadata(buf: &[u8], elf: &Elf) -> Result<HashMap<String, String>, String> {
    let mut meta = HashMap::new();
    meta.insert("Format".into(), "ELF".into());
    
    for sect in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(sect.sh_name) {
            if name == ".comment" {
                let offset = sect.sh_offset as usize;
                let size = sect.sh_size as usize;
                if offset + size <= buf.len() {
                    if let Ok(s) = std::str::from_utf8(&buf[offset..offset + size]) {
                        meta.insert("Comment".into(), s.trim_end_matches('\0').to_string());
                    }
                }
            }
        }
    }
    
    Ok(meta)
}
