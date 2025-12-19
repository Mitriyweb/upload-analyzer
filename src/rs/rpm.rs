use std::collections::HashMap;
use crate::{FileAnalyzer, MetadataResult};

pub struct RPMAnalyzer;

const RPM_LEAD_MAGIC: &[u8] = &[0xED, 0xAB, 0xEE, 0xDB];
const RPM_HEADER_MAGIC: &[u8] = &[0x8E, 0xAD, 0xE8, 0x01];

impl FileAnalyzer for RPMAnalyzer {
    fn get_file_info(_data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("Format".to_string(), "RPM".to_string());
        info
    }

    fn parse_metadata(data: &[u8]) -> MetadataResult {
        let mut meta = HashMap::new();
        meta.insert("Format".into(), "RPM".into());

        if data.len() < 96 {
            return Err("File too small for RPM Lead".into());
        }

        // RPM Lead is 96 bytes
        // We can extract basic info from lead if needed, but the real metadata is in the header

        let mut offset = 96;

        // Skip Signature Header
        offset = skip_header_structure(data, offset)?;

        // The next structure is the Immutable Header
        parse_header_structure(data, offset, &mut meta)?;


        Ok(meta)
    }
}

pub fn is_rpm_file(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == RPM_LEAD_MAGIC
}

fn skip_header_structure(data: &[u8], offset: usize) -> Result<usize, String> {
    if data.len() < offset + 16 {
        return Err("File too small for Header structure".into());
    }

    if &data[offset..offset + 4] != RPM_HEADER_MAGIC {
        return Err("Invalid RPM Header magic".into());
    }

    let index_count = u32::from_be_bytes([data[offset + 8], data[offset + 9], data[offset + 10], data[offset + 11]]) as usize;
    let store_size = u32::from_be_bytes([data[offset + 12], data[offset + 13], data[offset + 14], data[offset + 15]]) as usize;

    let total_size = 16 + (index_count * 16) + store_size;

    // Header structure is padded to 8 bytes
    let padded_size = (total_size + 7) & !7;

    Ok(offset + padded_size)
}

fn parse_header_structure(data: &[u8], offset: usize, meta: &mut HashMap<String, String>) -> Result<(), String> {
    if data.len() < offset + 16 {
        return Err("File too small for Immutable Header".into());
    }

    if &data[offset..offset + 4] != RPM_HEADER_MAGIC {
        return Err("Invalid RPM Immutable Header magic".into());
    }

    let index_count = u32::from_be_bytes([data[offset + 8], data[offset + 9], data[offset + 10], data[offset + 11]]) as usize;
    let store_size = u32::from_be_bytes([data[offset + 12], data[offset + 13], data[offset + 14], data[offset + 15]]) as usize;

    let index_start = offset + 16;
    let store_start = index_start + (index_count * 16);

    if data.len() < store_start + store_size {
        return Err("RPM file truncated in Header structure".into());
    }

    for i in 0..index_count {
        let entry_offset = index_start + (i * 16);
        let tag = u32::from_be_bytes([data[entry_offset], data[entry_offset + 1], data[entry_offset + 2], data[entry_offset + 3]]);
        let _dtype = u32::from_be_bytes([data[entry_offset + 4], data[entry_offset + 5], data[entry_offset + 6], data[entry_offset + 7]]);
        let offset = u32::from_be_bytes([data[entry_offset + 8], data[entry_offset + 9], data[entry_offset + 10], data[entry_offset + 11]]) as usize;
        // let count = u32::from_be_bytes([data[entry_offset + 12], data[entry_offset + 13], data[entry_offset + 14], data[entry_offset + 15]]) as usize;

        let abs_offset = store_start + offset;

        match tag {
            1000 => { // NAME
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("ProductName".into(), s);
                }
            }
            1001 => { // VERSION
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("ProductVersion".into(), s);
                }
            }
            1002 => { // RELEASE
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("Release".into(), s);
                }
            }
            1004 => { // SUMMARY
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("Description".into(), s);
                }
            }
            1011 => { // VENDOR
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("Vendor".into(), s);
                }
            }
            1014 => { // LICENSE
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("License".into(), s);
                }
            }
            1016 => { // GROUP
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("GroupName".into(), s);
                }
            }
            1020 => { // URL
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("Url".into(), s);
                }
            }
            1022 => { // ARCH
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("Architecture".into(), s);
                }
            }
            1044 => { // SOURCERPM
                if let Some(s) = read_string(data, abs_offset) {
                    meta.insert("SourceRpm".into(), s);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn read_string(data: &[u8], offset: usize) -> Option<String> {
    if offset >= data.len() {
        return None;
    }

    let mut end = offset;
    while end < data.len() && data[end] != 0 {
        end += 1;
    }

    String::from_utf8(data[offset..end].to_vec()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rpm_file() {
        let mut data = vec![0; 100];
        data[0..4].copy_from_slice(RPM_LEAD_MAGIC);
        assert!(is_rpm_file(&data));

        let invalid_data = vec![0; 100];
        assert!(!is_rpm_file(&invalid_data));
    }
}
