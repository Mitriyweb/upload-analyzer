mod msi;
mod pe;
mod dmg;

use goblin::Object;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

pub trait FileAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String>;
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String>;
}

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn parse_metadata(buf: &[u8]) -> Result<HashMap<String, String>, String> {
    if msi::is_msi_file(buf) {
        return msi::MSIAnalyzer::parse_metadata(buf);
    }
    
   
    if dmg::is_dmg_file(buf) {
        return dmg::DMGAnalyzer::parse_metadata(buf);
    }

    let obj = Object::parse(buf).map_err(|e| format!("Failed to parse file: {}", e))?;
    
    match obj {
        Object::PE(_) => pe::PEAnalyzer::parse_metadata(buf),
        _ => Err("Unsupported file format. Only PE, MSI, and DMG files are supported.".to_string())
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
        msi::MSIAnalyzer::get_file_info(data)
    } else if dmg::is_dmg_file(data) {
        dmg::DMGAnalyzer::get_file_info(data)
    } else if let Ok(obj) = Object::parse(data) {
        match obj {
            Object::PE(_) => pe::PEAnalyzer::get_file_info(data),
            _ => {
                let mut info = HashMap::new();
                info.insert("type".to_string(), "Unsupported".to_string());
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
