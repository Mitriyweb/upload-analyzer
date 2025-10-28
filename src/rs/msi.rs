use std::collections::HashMap;
use std::io::Cursor;
use cfb::CompoundFile;
use crate::FileAnalyzer;

pub struct MSIAnalyzer;

impl FileAnalyzer for MSIAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "MSI (Windows Installer)".to_string());
        info.insert("size".to_string(), data.len().to_string());
        info
    }
    
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
        parse_msi_metadata(data)
    }
}

pub fn is_msi_file(data: &[u8]) -> bool {
    data.len() >= 8 && data[0..8] == [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]
}

fn parse_msi_metadata(buf: &[u8]) -> Result<HashMap<String, String>, String> {
    let mut meta = HashMap::new();
    
    meta.insert("Format".into(), "MSI".into());
    meta.insert("Architecture".into(), "Windows Installer Package".into());
    
    let cursor = Cursor::new(buf);
    match CompoundFile::open(cursor) {
        Ok(mut cfb) => {
            meta.insert("HasCompoundFile".into(), "true".into());
            
            extract_summary_info(&mut cfb, &mut meta);
            
            extract_msi_properties(buf, &mut meta);
            
            // Extract additional metadata similar to PE files
            extract_msi_version_info(&mut meta);
            extract_language_from_summary(&mut cfb, &mut meta);
            extract_creation_date(&mut cfb, &mut meta);
            
        }
        Err(e) => {
            meta.insert("CompoundFileError".into(), format!("{:?}", e));
        }
    }
    
    Ok(meta)
}

fn extract_summary_info(cfb: &mut CompoundFile<Cursor<&[u8]>>, meta: &mut HashMap<String, String>) {
    if let Ok(mut stream) = cfb.open_stream("\u{0005}SummaryInformation") {
        use std::io::Read;
        let mut buffer = Vec::new();
        if stream.read_to_end(&mut buffer).is_ok() {
            meta.insert("HasSummaryInfo".into(), "true".into());
            
            extract_string_from_buffer(&buffer, meta, "Subject", "ProductName");
            extract_string_from_buffer(&buffer, meta, "Author", "Manufacturer");
            extract_string_from_buffer(&buffer, meta, "Comments", "Comments");
        }
    }
}

fn extract_string_from_buffer(buffer: &[u8], meta: &mut HashMap<String, String>, _key: &str, target: &str) {
    let mut current_string = String::new();
    let mut valid_strings: Vec<String> = Vec::new();
    
    for byte in buffer {
        if *byte >= 32 && *byte <= 126 {
            current_string.push(*byte as char);
        } else if *byte == 0 && current_string.len() >= 3 {
            if is_valid_metadata_string(&current_string) {
                valid_strings.push(current_string.clone());
            }
            current_string.clear();
        } else {
            current_string.clear();
        }
    }
    
    valid_strings.sort_by(|a, b| b.len().cmp(&a.len()));
    
    let best_string = valid_strings.iter().find(|s| {
        s.len() >= 5 && s.len() < 100 && 
        s.chars().filter(|c| c.is_alphabetic()).count() >= 3
    });
    
    if let Some(value) = best_string {
        meta.insert(target.into(), value.clone());
        
        match target {
            "Manufacturer" => {
                meta.insert("CompanyName".into(), value.clone());
                meta.insert("Publisher".into(), value.clone());
            }
            "ProductName" => {
                meta.insert("Product".into(), value.clone());
            }
            _ => {}
        }
    }
}

fn is_valid_metadata_string(s: &str) -> bool {
    if s.len() < 3 || s.len() > 100 {
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
    
    if let Some(version) = extract_version_pattern(&buf_str) {
        meta.insert("ProductVersion".into(), version.clone());
        meta.insert("Version".into(), version);
    }
    
    if !meta.contains_key("Manufacturer") {
        if let Some(manufacturer) = extract_property_value(buf, b"Manufacturer") {
            meta.insert("Manufacturer".into(), manufacturer.clone());
            meta.insert("CompanyName".into(), manufacturer.clone());
            meta.insert("Publisher".into(), manufacturer);
        }
    }
    
    if !meta.contains_key("ProductName") {
        if let Some(product_name) = extract_property_value(buf, b"ProductName") {
            meta.insert("ProductName".into(), product_name.clone());
            meta.insert("Product".into(), product_name);
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
            if byte >= 32 && byte <= 126 && byte != b'\\' {
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

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn extract_guid(data: &str, _prefix: &str) -> Option<String> {
    let guid_pattern = regex_like_guid_search(data.as_bytes());
    guid_pattern
}

fn regex_like_guid_search(data: &[u8]) -> Option<String> {
    for i in 0..data.len().saturating_sub(38) {
        if data[i] == b'{' && data[i + 37] == b'}' {
            if data[i + 9] == b'-' && data[i + 14] == b'-' && 
               data[i + 19] == b'-' && data[i + 24] == b'-' {
                if let Ok(guid) = std::str::from_utf8(&data[i..i+38]) {
                    if guid.chars().all(|c| c.is_ascii_hexdigit() || c == '{' || c == '}' || c == '-') {
                        return Some(guid.to_uppercase());
                    }
                }
            }
        }
    }
    None
}

fn extract_version_pattern(data: &str) -> Option<String> {
    let bytes = data.as_bytes();
    
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
            if parts.len() >= 2 && parts.len() <= 4 {
                if parts.iter().all(|p| !p.is_empty() && p.parse::<u32>().is_ok()) {
                    return Some(version);
                }
            }
        }
    }
    
    None
}

fn extract_msi_version_info(meta: &mut HashMap<String, String>) {
    // Create aliases similar to PE files
    if let Some(product_version) = meta.get("ProductVersion").cloned() {
        // Parse version number components
        let parts: Vec<&str> = product_version.split('.').collect();
        if parts.len() >= 2 {
            meta.insert("FileVersion".into(), product_version.clone());
            meta.insert("FileVersionNumber".into(), product_version.clone());
            meta.insert("ProductVersionNumber".into(), product_version);
        }
    }
    
    // Create ProgramName alias from ProductName
    if let Some(product_name) = meta.get("ProductName").cloned() {
        meta.insert("ProgramName".into(), product_name);
    }
    
    // Create Vendor alias from Manufacturer
    if let Some(manufacturer) = meta.get("Manufacturer").cloned() {
        if !meta.contains_key("Vendor") {
            meta.insert("Vendor".into(), manufacturer);
        }
    }
    
    // Add file description from ProductName or Comments
    if !meta.contains_key("FileDescription") {
        if let Some(product_name) = meta.get("ProductName").cloned() {
            if let Some(manufacturer) = meta.get("Manufacturer") {
                if !manufacturer.is_empty() {
                    meta.insert("FileDescription".into(), format!("{} Installer", product_name));
                }
            }
        } else if let Some(comments) = meta.get("Comments").cloned() {
            meta.insert("FileDescription".into(), comments);
        }
    }
}

fn extract_language_from_summary(cfb: &mut CompoundFile<Cursor<&[u8]>>, meta: &mut HashMap<String, String>) {
    use std::io::Read;
    
    if let Ok(mut stream) = cfb.open_stream("\u{0005}SummaryInformation") {
        let mut buffer = Vec::new();
        if stream.read_to_end(&mut buffer).is_ok() {
            // Try to extract language code from summary info
            // Summary info contains language as a 16-bit value
            // Common LCIDs: 1033 (en-US), 1031 (de-DE), 1036 (fr-FR)
            
            // Look for language patterns in the buffer
            for i in 0..buffer.len().saturating_sub(10) {
                // Check for language indicator patterns
                if buffer[i] == 0x09 && buffer[i+1] == 0x04 { // 1033 in little-endian
                    meta.insert("Language".into(), "English (United States)".into());
                    meta.insert("LanguageCode".into(), "1033".into());
                    break;
                } else if buffer[i] == 0x07 && buffer[i+1] == 0x04 { // 1031
                    meta.insert("Language".into(), "German (Germany)".into());
                    meta.insert("LanguageCode".into(), "1031".into());
                    break;
                } else if buffer[i] == 0x0C && buffer[i+1] == 0x04 { // 1036
                    meta.insert("Language".into(), "French (France)".into());
                    meta.insert("LanguageCode".into(), "1036".into());
                    break;
                }
            }
        }
    }
}

fn extract_creation_date(cfb: &mut CompoundFile<Cursor<&[u8]>>, meta: &mut HashMap<String, String>) {
    use std::io::Read;
    
    if let Ok(mut stream) = cfb.open_stream("\u{0005}SummaryInformation") {
        let mut buffer = Vec::new();
        if stream.read_to_end(&mut buffer).is_ok() {
            // Summary info structure contains creation/modification timestamps
            // Try to extract readable date information
            
            // Look for FILETIME structures (8 bytes, little-endian)
            // This is a simplified extraction - proper parsing would decode FILETIME format
            
            // For now, just indicate that timestamps are present
            if buffer.len() >= 48 {
                meta.insert("HasTimestamp".into(), "true".into());
            }
        }
    }
}
