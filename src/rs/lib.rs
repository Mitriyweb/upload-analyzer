mod msi;
mod pe;
mod elf;
mod macho;

use goblin::Object;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde_json;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn parse_metadata(buf: &[u8]) -> Result<HashMap<String, String>, String> {
    if msi::is_msi_file(buf) {
        return msi::parse_msi_metadata(buf);
    }

    let obj = Object::parse(&buf).map_err(|e| format!("Failed to parse file: {}", e))?;
    
    match obj {
        Object::PE(pe) => pe::parse_pe_metadata(buf, &pe),
        Object::Elf(elf) => elf::parse_elf_metadata(buf, &elf),
        Object::Mach(mach) => macho::parse_macho_metadata(&mach),
        _ => Ok(HashMap::new())
    }
}

#[wasm_bindgen]
pub fn analyze_file(data: &[u8]) -> String {
    match parse_metadata(data) {
        Ok(meta) => serde_json::to_string(&meta).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => format!("{{\"error\": \"{}\"}}", e)
    }
}

#[wasm_bindgen]
pub fn get_file_info(data: &[u8]) -> String {
    let info = if msi::is_msi_file(data) {
        msi::get_file_info(data)
    } else if let Ok(obj) = Object::parse(data) {
        match obj {
            Object::PE(_) => pe::get_file_info(data),
            Object::Elf(_) => elf::get_file_info(data),
            Object::Mach(_) => macho::get_file_info(data),
            _ => {
                let mut info = HashMap::new();
                info.insert("type".to_string(), "Unknown".to_string());
                info
            }
        }
    } else {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "Invalid binary".to_string());
        info
    };
    
    serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn analyze_pe_file(data: &[u8]) -> String {
    match parse_metadata(data) {
        Ok(meta) => serde_json::to_string(&meta).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => format!("{{\"error\": \"{}\"}}", e)
    }
}
