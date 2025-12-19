mod msi;
mod pe;
mod dmg;
mod deb;
mod rpm;

use goblin::Object;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Type alias to reduce complexity and improve readability
pub type MetadataResult = Result<HashMap<String, String>, String>;

pub trait FileAnalyzer {
    fn get_file_info(_data: &[u8]) -> HashMap<String, String>;
    fn parse_metadata(data: &[u8]) -> MetadataResult;
}

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn parse_metadata(buf: &[u8]) -> MetadataResult {
    if msi::is_msi_file(buf) {
        return msi::MSIAnalyzer::parse_metadata(buf);
    }


    if dmg::is_dmg_file(buf) {
        return dmg::DMGAnalyzer::parse_metadata(buf);
    }

    if deb::is_deb_file(buf) {
        return deb::DEBAnalyzer::parse_metadata(buf);
    }

    if rpm::is_rpm_file(buf) {
        return rpm::RPMAnalyzer::parse_metadata(buf);
    }

    let obj = Object::parse(buf).map_err(|e| format!("Failed to parse file: {}", e))?;

    match obj {
        Object::PE(_) => pe::PEAnalyzer::parse_metadata(buf),
        _ => Err("Unsupported file format. Supported formats: PE, MSI, DMG, DEB, RPM.".to_string())
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
    let mut info = if msi::is_msi_file(data) {
        msi::MSIAnalyzer::get_file_info(data)
    } else if dmg::is_dmg_file(data) {
        dmg::DMGAnalyzer::get_file_info(data)
    } else if deb::is_deb_file(data) {
        deb::DEBAnalyzer::get_file_info(data)
    } else if rpm::is_rpm_file(data) {
        rpm::RPMAnalyzer::get_file_info(data)
    } else if let Ok(obj) = Object::parse(data) {
        match obj {
            Object::PE(_) => pe::PEAnalyzer::get_file_info(data),
            _ => {
                let mut info = HashMap::new();
                info.insert("Format".to_string(), "Unsupported".to_string());
                info
            }
        }
    } else {
        let mut info = HashMap::new();
        info.insert("Format".to_string(), "Invalid binary".to_string());
        info
    };

    info.insert("Size".to_string(), data.len().to_string());

    serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
}

#[wasm_bindgen]
pub fn analyze_pe_file(data: &[u8]) -> String {
    match parse_metadata(data) {
        Ok(meta) => serde_json::to_string(&meta).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => format!("{{\"error\": \"{}\"}}", e)
    }
}
