use std::collections::HashMap;
use std::io::{Cursor, Read};
use cfb::CompoundFile;
use crate::{FileAnalyzer, MetadataResult};

// Constants for MSI file analysis
const MSI_SIGNATURE: &[u8] = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];
const MIN_MSI_SIGNATURE_SIZE: usize = 8;
const MIN_METADATA_STRING_LEN: usize = 3;
const MAX_METADATA_STRING_LEN: usize = 100;

// Type alias to reduce complexity
type CfbFile<'a> = CompoundFile<Cursor<&'a [u8]>>;

pub struct MSIAnalyzer;

impl FileAnalyzer for MSIAnalyzer {
    fn get_file_info(_data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("Format".to_string(), "MSI".to_string());
        info
    }

    fn parse_metadata(data: &[u8]) -> MetadataResult {
        parse_msi_metadata(data)
    }
}

pub fn is_msi_file(data: &[u8]) -> bool {
    data.len() >= MIN_MSI_SIGNATURE_SIZE && &data[0..MIN_MSI_SIGNATURE_SIZE] == MSI_SIGNATURE
}

struct MsiTableReader<'a> {
    data: &'a [u8],
    row_size: usize,
}

impl<'a> MsiTableReader<'a> {
    fn new(data: &'a [u8], row_size: usize) -> Self {
        Self { data, row_size }
    }

    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.data.chunks_exact(self.row_size)
    }
}

fn parse_msi_metadata(buf: &[u8]) -> MetadataResult {
    let mut meta = HashMap::new();
    meta.insert("Format".into(), "MSI".into());

    let cursor = Cursor::new(buf);
    let mut cfb = match CompoundFile::open(cursor) {
        Ok(cfb) => cfb,
        Err(e) => {
            // Fallback to heuristics if CFB fails
            extract_msi_properties(buf, &mut meta);
            meta.insert("CompoundFileError".into(), format!("{:?}", e));
            return Ok(meta);
        }
    };

    // 1. Extract String Pool
    let mut string_pool = None;
    let mut pool_data = Vec::new();
    let mut data_data = Vec::new();

    let storage_entries: Vec<_> = match cfb.read_storage("/") {
        Ok(storage) => storage.collect(),
        Err(_) => {
            extract_msi_properties(buf, &mut meta);
            return Ok(meta);
        }
    };

    for entry in &storage_entries {
        let decoded_name = decode_msi_stream_name(entry.name());
        if decoded_name == "!StringPool" {
            if let Ok(mut stream) = cfb.open_stream(entry.path()) {
                let _ = stream.read_to_end(&mut pool_data);
            }
        } else if decoded_name == "!StringData" {
            if let Ok(mut stream) = cfb.open_stream(entry.path()) {
                let _ = stream.read_to_end(&mut data_data);
            }
        }
    }

    if !pool_data.is_empty() && !data_data.is_empty() {
        string_pool = Some(MsiStringPool::from_streams(&pool_data, &data_data));
    }

    // 2. Extract Property Table
    if let Some(ref pool) = string_pool {
        let idx_size = pool.index_size;
        for entry in &storage_entries {
            let decoded_name = decode_msi_stream_name(entry.name());
            let name = decoded_name.trim_start_matches('!').trim_start_matches('\u{0005}');

            match name {
                "Property" => {
                    if let Ok(mut stream) = cfb.open_stream(entry.path()) {
                        let mut prop_data = Vec::new();
                        if stream.read_to_end(&mut prop_data).is_ok() {
                            let row_size = idx_size * 2;
                            let reader = MsiTableReader::new(&prop_data, row_size);
                            for row in reader.rows() {
                                let key_idx = read_idx(row, 0, idx_size);
                                let val_idx = read_idx(row, idx_size, idx_size);
                                if let (Some(key), Some(val)) = (pool.get(key_idx), pool.get(val_idx)) {
                                    if !key.is_empty() && !val.is_empty() {
                                        meta.insert(key.clone(), val.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                "File" => {
                    // File table row size: 8 bytes + 5 strings
                    let row_size = (idx_size * 5) + 8;
                    let n_rows = entry.len() / (row_size as u64);
                    meta.insert("FileCount".into(), n_rows.to_string());

                    if let Ok(mut stream) = cfb.open_stream(entry.path()) {
                        let mut file_data = Vec::new();
                        if stream.read_to_end(&mut file_data).is_ok() {
                            let mut total_size: u64 = 0;
                            for row in file_data.chunks_exact(row_size) {
                                let size_offset = idx_size * 3;
                                if row.len() >= size_offset + 4 {
                                    let size = u32::from_le_bytes([
                                        row[size_offset],
                                        row[size_offset + 1],
                                        row[size_offset + 2],
                                        row[size_offset + 3]
                                    ]) as u64;
                                    total_size += size;
                                }
                            }
                            meta.insert("TotalFileSize".into(), total_size.to_string());
                        }
                    }
                }
                "Component" => {
                    // Component row: 5 strings + 2 bytes
                    let row_size = (idx_size * 5) + 2;
                    meta.insert("ComponentCount".into(), (entry.len() / (row_size as u64)).to_string());
                }
                "Feature" => {
                    // Feature row: 5 strings + 6 bytes
                    let row_size = (idx_size * 5) + 6;
                    meta.insert("FeatureCount".into(), (entry.len() / (row_size as u64)).to_string());
                }
                "LaunchCondition" => {
                    if let Ok(mut stream) = cfb.open_stream(entry.path()) {
                        let mut lc_data = Vec::new();
                        if stream.read_to_end(&mut lc_data).is_ok() {
                            let row_size = idx_size * 2;
                            let reader = MsiTableReader::new(&lc_data, row_size);
                            let mut conditions = Vec::new();
                            for row in reader.rows() {
                                let val_idx = read_idx(row, idx_size, idx_size);
                                if let Some(val) = pool.get(val_idx) {
                                    conditions.push(val.clone());
                                }
                            }
                            if !conditions.is_empty() {
                                meta.insert("LaunchConditions".into(), conditions.join(" | "));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // 3. Extract Summary Information (Standard OLE)
    extract_summary_info_enhanced(&mut cfb, &mut meta);

    // 4. Manual Fallbacks

    // Heuristic fallbacks for anything missing
    if !meta.contains_key("ProductName") || !meta.contains_key("ProductVersion") {
        extract_msi_properties(buf, &mut meta);
    }

    Ok(meta)
}

fn read_idx(data: &[u8], offset: usize, size: usize) -> usize {
    if data.len() < offset + size {
        return 0;
    }
    match size {
        2 => u16::from_le_bytes([data[offset], data[offset + 1]]) as usize,
        3 => {
            (data[offset] as usize) |
            ((data[offset + 1] as usize) << 8) |
            ((data[offset + 2] as usize) << 16)
        },
        _ => 0,
    }
}

fn extract_summary_info_enhanced(cfb: &mut CfbFile, meta: &mut HashMap<String, String>) {
    if let Ok(mut stream) = cfb.open_stream("\u{0005}SummaryInformation") {
        let mut buffer = Vec::new();
        if stream.read_to_end(&mut buffer).is_ok() {
            extract_ole_properties(&buffer, meta);
        }
    }
}

fn get_u32(buf: &[u8], offset: usize) -> u32 {
    if offset + 4 > buf.len() { return 0; }
    u32::from_le_bytes([buf[offset], buf[offset+1], buf[offset+2], buf[offset+3]])
}

fn get_u16(buf: &[u8], offset: usize) -> u16 {
    if offset + 2 > buf.len() { return 0; }
    u16::from_le_bytes([buf[offset], buf[offset+1]])
}

fn extract_ole_properties(buffer: &[u8], meta: &mut HashMap<String, String>) {
    if buffer.len() < 48 || get_u16(buffer, 0) != 0xFFFE {
        return;
    }

    let num_sections = get_u32(buffer, 24);
    if num_sections == 0 { return; }

    let section_offset = get_u32(buffer, 44) as usize;
    if section_offset + 8 > buffer.len() { return; }

    let section_size = get_u32(buffer, section_offset) as usize;
    let prop_count = get_u32(buffer, section_offset + 4) as usize;

    if section_offset + section_size > buffer.len() { return; }

    let entry_base = section_offset + 8;
    for i in 0..prop_count {
        let entry_offset = entry_base + (i * 8);
        if entry_offset + 8 > buffer.len() { break; }

        let pid = get_u32(buffer, entry_offset);
        let prop_offset = get_u32(buffer, entry_offset + 4) as usize;
        let abs_prop_offset = section_offset + prop_offset;

        if abs_prop_offset + 4 > buffer.len() { continue; }

        let prop_type = get_u16(buffer, abs_prop_offset);

        match pid {
            2 | 3 | 4 | 5 | 6 | 9 => {
                let s = if prop_type == 30 { // VT_LPSTR
                    let str_len = get_u32(buffer, abs_prop_offset + 4) as usize;
                    let str_start = abs_prop_offset + 8;
                    if str_start + str_len <= buffer.len() {
                        String::from_utf8_lossy(&buffer[str_start..str_start + str_len])
                            .trim_matches(char::from(0))
                            .to_string()
                    } else {
                        continue;
                    }
                } else if prop_type == 31 { // VT_LPWSTR (UTF-16)
                    let str_chars = get_u32(buffer, abs_prop_offset + 4) as usize;
                    let str_start = abs_prop_offset + 8;
                    if str_start + (str_chars * 2) <= buffer.len() {
                        let utf16_data: Vec<u16> = buffer[str_start..str_start + (str_chars * 2)]
                            .chunks_exact(2)
                            .map(|c| u16::from_le_bytes([c[0], c[1]]))
                            .collect();
                        String::from_utf16_lossy(&utf16_data)
                            .trim_matches(char::from(0))
                            .to_string()
                    } else {
                        continue;
                    }
                } else {
                    continue;
                };

                if s.is_empty() { continue; }

                let key = match pid {
                    2 => "Title",
                    3 => "ProductName",
                    4 => "Manufacturer",
                    5 => "Keywords",
                    6 => "Comments",
                    9 => "PackageCode",
                    _ => continue,
                };

                // Only overwrite if it's a primary summary field or if structured extraction was empty
                if pid == 2 || pid == 5 || pid == 6 || pid == 9 || !meta.contains_key(key) {
                    meta.insert(key.into(), s);
                }
            }
            _ => {}
        }
    }
}

fn is_valid_metadata_string(s: &str) -> bool {
    if s.len() < MIN_METADATA_STRING_LEN || s.len() > MAX_METADATA_STRING_LEN {
        return false;
    }

    if !s.chars().any(|c| c.is_alphabetic()) {
        return false;
    }

    let blacklist = [
        "Installation Database",
        "Installer Database",
        "Windows Installer",
        "Microsoft Corporation",
        "MsiExec",
        "Property",
        "Feature",
        "Component",
        "Directory",
        "Registry",
        "AdminExecuteSequence",
        "InstallExecuteSequence",
        "ProductCode",
        "UpgradeCode",
        "TARGETDIR",
        "ProgramFilesFolder",
    ];

    for blocked in &blacklist {
        if s.contains(blocked) {
            return false;
        }
    }

    let valid_count = s.chars().filter(|c| {
        c.is_alphanumeric() || c.is_whitespace() ||
        *c == '.' || *c == '-' || *c == '_' || *c == ',' ||
        *c == '(' || *c == ')' || *c == '&' || *c == '\''
    }).count();

    valid_count == s.len()
}

fn extract_msi_properties(buf: &[u8], meta: &mut HashMap<String, String>) {
    let buf_str = String::from_utf8_lossy(buf);

    if let Some(product_code) = extract_guid(&buf_str, "ProductCode") {
        meta.insert("ProductCode".into(), product_code);
    }

    if let Some(upgrade_code) = extract_guid(&buf_str, "UpgradeCode") {
        meta.insert("UpgradeCode".into(), upgrade_code);
    }

    if !meta.contains_key("ProductVersion") {
        if let Some(version) = extract_version_pattern(&buf_str) {
            meta.insert("ProductVersion".into(), version);
        }
    }

    if !meta.contains_key("Manufacturer") {
        if let Some(manufacturer) = extract_property_value(buf, b"Manufacturer") {
            meta.insert("Manufacturer".into(), manufacturer);
        }
    }

    if !meta.contains_key("ProductName") {
        if let Some(product_name) = extract_property_value(buf, b"ProductName") {
            meta.insert("ProductName".into(), product_name);
        }
    }

    if buf_str.contains("WixToolset") || buf_str.contains("Windows Installer XML") {
        meta.insert("InstallerFramework".into(), "WiX Toolset".into());
    } else if buf_str.contains("InstallShield") {
        meta.insert("InstallerFramework".into(), "InstallShield".into());
    } else if buf_str.contains("Advanced Installer") {
        meta.insert("InstallerFramework".into(), "Advanced Installer".into());
    }
}

fn extract_property_value(buf: &[u8], property_name: &[u8]) -> Option<String> {
    if let Some(pos) = find_bytes(buf, property_name) {
        let start = pos + property_name.len();
        let end = (start + 200).min(buf.len());
        let search_area = &buf[start..end];

        let mut found_string = String::new();
        let mut in_string = false;

        for &byte in search_area {
            if (32..=126).contains(&byte) && byte != b'\\' {
                found_string.push(byte as char);
                in_string = true;
            } else if in_string && found_string.len() >= 3 {
                if is_valid_metadata_string(&found_string) {
                    return Some(found_string);
                }
                found_string.clear();
                in_string = false;
            } else {
                found_string.clear();
                in_string = false;
            }
        }
    }
    None
}

#[inline]
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn extract_guid(data: &str, _prefix: &str) -> Option<String> {
    let guid_pattern = regex_like_guid_search(data.as_bytes());
    guid_pattern
}

fn regex_like_guid_search(data: &[u8]) -> Option<String> {
    for i in 0..data.len().saturating_sub(38) {
        if data[i] == b'{' && data[i + 37] == b'}'
            && data[i + 9] == b'-' && data[i + 14] == b'-' &&
               data[i + 19] == b'-' && data[i + 24] == b'-' {
                if let Ok(guid) = std::str::from_utf8(&data[i..i+38]) {
                    if guid.chars().all(|c| c.is_ascii_hexdigit() || c == '{' || c == '}' || c == '-') {
                        return Some(guid.to_uppercase());
                    }
                }
            }
    }
    None
}

fn extract_version_pattern(data: &str) -> Option<String> {
    let bytes = data.as_bytes();
    let mut versions = Vec::new();

    for i in 0..bytes.len().saturating_sub(5) {
        if bytes[i].is_ascii_digit() && bytes[i + 1] == b'.' && bytes[i + 2].is_ascii_digit() {
            let mut version = String::new();
            let mut j = i;

            while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b'.') {
                version.push(bytes[j] as char);
                j += 1;

                if version.len() > 20 {
                    break;
                }
            }

            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() >= 2 && parts.len() <= 4
                && parts.iter().all(|p| !p.is_empty() && p.parse::<u32>().is_ok()) {
                    versions.push(version);
                }
        }
    }

    // Prefer versions with more dots and not starting with "3." if others exist (common build numbers vs product version)
    versions.sort_by_key(|v| {
        let dots = v.chars().filter(|&c| c == '.').count();
        let is_low_major = v.starts_with("3."); // Heuristic: many MSIs have internal version 3.x
        (dots, !is_low_major)
    });

    versions.pop()
}

struct MsiStringPool {
    strings: Vec<String>,
    index_size: usize,
}

impl MsiStringPool {
    fn from_streams(pool_data: &[u8], string_data: &[u8]) -> Self {
        if pool_data.len() < 4 {
            return Self { strings: Vec::new(), index_size: 2 };
        }

        let codepage = u16::from_le_bytes([pool_data[0], pool_data[1]]);
        let flags = u16::from_le_bytes([pool_data[2], pool_data[3]]);
        let index_size = if (flags & 0x8000) != 0 { 3 } else { 2 };

        let n_entries = (pool_data.len() - 4) / 4;
        let mut strings = Vec::with_capacity(n_entries);
        let mut current_offset = 0;

        for i in 0..n_entries {
            let offset = 4 + (i * 4);
            let length = u16::from_le_bytes([pool_data[offset + 2], pool_data[offset + 3]]) as usize;

            if length == 0 {
                strings.push(String::new());
                continue;
            }

            let end = current_offset + length;
            if end <= string_data.len() {
                let s_bytes = &string_data[current_offset..end];
                let s = if codepage == 1200 {
                    let utf16_data: Vec<u16> = s_bytes.chunks_exact(2)
                        .map(|c| u16::from_le_bytes([c[0], c[1]]))
                        .collect();
                    String::from_utf16_lossy(&utf16_data).to_string()
                } else {
                    String::from_utf8_lossy(s_bytes).to_string()
                };
                strings.push(s);
            } else {
                strings.push(String::new());
            }
            current_offset += length;
        }

        Self { strings, index_size }
    }

    fn get(&self, index: usize) -> Option<&String> {
        if index == 0 || index > self.strings.len() {
            return None;
        }
        Some(&self.strings[index - 1])
    }
}

fn decode_msi_stream_name(name: &str) -> String {
    if name.starts_with('!') || name.starts_with('\u{0005}') {
        return name.to_string();
    }

    let mut decoded = String::new();
    for c in name.chars() {
        let n = c as u32;
        if (0x3800..0x4840).contains(&n) {
            let n = n - 0x3800;
            let char1 = (n & 0x3F) as u8;
            let char2 = ((n >> 6) & 0x3F) as u8;

            decoded.push(decode_char(char1));
            if char2 != 0 {
                decoded.push(decode_char(char2));
            }
        } else {
            decoded.push(c);
        }
    }
    decoded
}

fn decode_char(c: u8) -> char {
    match c {
        0..=9 => (b'0' + c) as char,
        10..=35 => (b'a' + (c - 10)) as char,
        36..=61 => (b'A' + (c - 36)) as char,
        62 => '_',
        63 => '.',
        _ => ' ',
    }
}

#[cfg(test)]
mod msi_tests {
    use super::*;

    #[test]
    fn test_decode_msi_stream_name() {
        // "Property" table often encodes to specific mangled name
        let _mangled_property = "\u{3EF3}\u{3E30}\u{3E32}\u{3E39}";

        let mangled = "\u{3EF3}\u{3E58}\u{3ECE}\u{409D}";
        assert_eq!(decode_msi_stream_name(mangled), "Property");
    }

    #[test]
    fn test_msi_string_pool() {
        // Header: 0, 0, 0, 0 (n_entries, flags)
        // Entry 1: refcount 1, len 5
        // Entry 2: refcount 1, len 5
        let pool = vec![
            0, 0, 0, 0, // Header (n_entries=0, flags=0) - n_entries is calculated from pool_data.len()
            1, 0, 5, 0, // Entry 1: refcount 1, len 5
            1, 0, 5, 0, // Entry 2: refcount 1, len 5
        ];
        let data = b"Test1Test2";
        let sp = MsiStringPool::from_streams(&pool, data);

        assert_eq!(sp.get(1).map(|s| s.as_str()), Some("Test1"));
        assert_eq!(sp.get(2).map(|s| s.as_str()), Some("Test2"));
        assert!(sp.get(0).is_none());
        assert!(sp.get(3).is_none());
    }
}
