use std::collections::HashMap;
use std::io::Cursor;
use crate::FileAnalyzer;
use plist::Value;

// Constants for DMG file analysis
const DMG_KOLY_SIGNATURE: &[u8] = b"koly";
const DMG_KOLY_OFFSET_SIZE: usize = 512;
const MIN_DMG_SIZE: usize = 512;

pub struct DMGAnalyzer;

impl FileAnalyzer for DMGAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "DMG (Apple Disk Image)".to_string());
        info.insert("size".to_string(), data.len().to_string());
        info
    }
    
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
        parse_dmg_metadata(data)
    }
}

pub fn is_dmg_file(data: &[u8]) -> bool {
    if data.len() < MIN_DMG_SIZE {
        return false;
    }
    
    if data.len() >= DMG_KOLY_OFFSET_SIZE {
        let end_offset = data.len() - DMG_KOLY_OFFSET_SIZE;
        
        if &data[end_offset..end_offset + 4] == DMG_KOLY_SIGNATURE {
            return true;
        }
    }
    
    if data.len() >= 4
        && (data[0..4] == [0x78, 0x01, 0x73, 0x0D] ||
           data[0..4] == [0x78, 0x9C, 0xEC, 0xBD] ||
           data[0..4] == [0x78, 0x9C, 0x00, 0x00] ||
           data[0..2] == [0x78, 0x01] ||
           data[0..2] == [0x78, 0x5E] ||
           data[0..2] == [0x78, 0x9C] ||
           data[0..2] == [0x78, 0xDA] ||
           data[0..2] == [0x1F, 0x8B] ||
           data[0..4] == [0x42, 0x5A, 0x68, 0x39] ||
           data[0..4] == [0x42, 0x5A, 0x68, 0x31])
    {
        if data.len() >= DMG_KOLY_OFFSET_SIZE {
            let end_offset = data.len() - DMG_KOLY_OFFSET_SIZE;
            if &data[end_offset..end_offset + 4] == DMG_KOLY_SIGNATURE {
                return true;
            }
        }
    }
    
    false
}

fn parse_dmg_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
    let mut meta = HashMap::new();
    
    meta.insert("Format".into(), "DMG".into());
    meta.insert("Architecture".into(), "macOS Disk Image".into());
    
    if data.len() >= 4 {
        let compression = if data[0..2] == [0x78, 0x01] || 
                            data[0..2] == [0x78, 0x5E] || 
                            data[0..2] == [0x78, 0x9C] || 
                            data[0..2] == [0x78, 0xDA] {
            "zlib"
        } else if data[0..2] == [0x1F, 0x8B] {
            "gzip"
        } else if data[0..4] == [0x42, 0x5A, 0x68, 0x39] || 
                  data[0..4] == [0x42, 0x5A, 0x68, 0x31] {
            "bzip2"
        } else if data[0] == 0x00 && data[1] == 0x00 {
            "uncompressed"
        } else {
            "unknown"
        };
        
        meta.insert("Compression".into(), compression.into());
    }
    
    if data.len() >= 512 {
        let koly_offset = data.len() - 512;
        
        if &data[koly_offset..koly_offset + 4] == b"koly" {
            meta.insert("HasKolySignature".into(), "true".into());
            meta.insert("KolyOffset".into(), koly_offset.to_string());
            
            if koly_offset + 8 <= data.len() {
                let version = u32::from_be_bytes([
                    data[koly_offset + 4],
                    data[koly_offset + 5],
                    data[koly_offset + 6],
                    data[koly_offset + 7]
                ]);
                meta.insert("DMGVersion".into(), version.to_string());
            }
        }
    }
    
    meta.insert("ImageType".into(), "UDIF".into());
    
    extract_product_info(data, &mut meta);
    
    Ok(meta)
}

fn extract_product_info(data: &[u8], meta: &mut HashMap<String, String>) {
    if let Some(plist_data) = find_plist_in_dmg(data) {
        parse_plist_properly(&plist_data, meta);
    }
    
    if !meta.contains_key("ProductName") || !meta.contains_key("ProductVersion") {
        let data_str = String::from_utf8_lossy(data);
        extract_plist_info(&data_str, meta);
        extract_version_strings(&data_str, meta);
        extract_bundle_info(&data_str, meta);
        extract_developer_info(&data_str, meta);
        
        if !meta.contains_key("ProductName") {
            extract_app_names(data, meta);
        }
    }
    
    create_field_aliases(meta);
}

fn find_plist_in_dmg(data: &[u8]) -> Option<Vec<u8>> {
    let data_str = String::from_utf8_lossy(data);
    
    if let Some(info_plist_pos) = data_str.find("Contents/Info.plist") {
        let search_start = info_plist_pos.saturating_sub(100000).max(0);
        let search_end = (info_plist_pos + 100000).min(data.len());
        let search_region = &data[search_start..search_end];
        
        if let Some(plist_data) = find_plist_in_region(search_region) {
            return Some(plist_data);
        }
    }
    
    find_plist_in_region(data)
}

fn find_plist_in_region(data: &[u8]) -> Option<Vec<u8>> {
    let xml_markers: &[&[u8]] = &[
        b"<?xml version=\"1.0\"",
        b"<plist version=",
        b"<!DOCTYPE plist",
    ];
    
    let binary_marker = b"bplist";
    
    for marker in xml_markers {
        if let Some(pos) = find_bytes(data, marker) {
            if let Some(end_pos) = find_bytes(&data[pos..], b"</plist>") {
                let plist_data = &data[pos..pos + end_pos + 8];
                
                let plist_str = String::from_utf8_lossy(plist_data);
                if plist_str.contains("CFBundleName") || 
                   plist_str.contains("CFBundleIdentifier") ||
                   plist_str.contains("CFBundleVersion") {
                    return Some(plist_data.to_vec());
                }
            }
        }
    }
    
    if let Some(pos) = find_bytes(data, binary_marker) {
        let end = (pos + 50000).min(data.len());
        return Some(data[pos..end].to_vec());
    }
    
    None
}

#[inline]
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn parse_plist_properly(plist_data: &[u8], meta: &mut HashMap<String, String>) {
    let value = Value::from_reader_xml(plist_data)
        .or_else(|_| {
            let cursor = Cursor::new(plist_data);
            Value::from_reader(cursor)
        });
    
    if let Ok(Value::Dictionary(dict)) = value {
        let keys_to_extract = [
            ("CFBundleName", "ProductName"),
            ("CFBundleDisplayName", "DisplayName"),
            ("CFBundleExecutable", "ExecutableName"),
            ("CFBundleIdentifier", "BundleIdentifier"),
            ("CFBundleShortVersionString", "ProductVersion"),
            ("CFBundleVersion", "FileVersion"),
            ("NSHumanReadableCopyright", "LegalCopyright"),
            ("CFBundleGetInfoString", "FileDescription"),
            ("LSApplicationCategoryType", "ApplicationCategory"),
            ("CFBundlePackageType", "PackageType"),
            ("NSPrincipalClass", "PrincipalClass"),
            ("CFBundleIconFile", "IconFile"),
            ("LSMinimumSystemVersion", "MinimumSystemVersion"),
        ];
        
        for (plist_key, meta_key) in &keys_to_extract {
            if let Some(Value::String(s)) = dict.get(plist_key) {
                let value = s.trim();
                if !value.is_empty() {
                    if *meta_key == "ApplicationCategory" {
                        let clean = value
                            .split('.').next_back().unwrap_or(value)
                            .replace("-", " ")
                            .split_whitespace()
                            .map(capitalize_first)
                            .collect::<Vec<_>>()
                            .join(" ");
                        meta.insert(meta_key.to_string(), clean);
                    } else {
                        meta.insert(meta_key.to_string(), value.to_string());
                    }
                }
            }
        }
        
        if !meta.contains_key("ProductName") && meta.contains_key("DisplayName") {
            meta.insert("ProductName".into(), meta.get("DisplayName").unwrap().clone());
        }
        
        if meta.contains_key("ProductVersion") && !meta.contains_key("FileVersion") {
            meta.insert("FileVersion".into(), meta.get("ProductVersion").unwrap().clone());
        }
        
        if !meta.contains_key("CompanyName") {
            if let Some(bundle_id) = meta.get("BundleIdentifier") {
                let parts: Vec<&str> = bundle_id.split('.').collect();
                if parts.len() >= 2 {
                    let company = parts[1];
                    if !company.is_empty() && company.chars().all(|c| c.is_alphanumeric()) {
                        meta.insert("CompanyName".into(), capitalize_first(company));
                    }
                }
            }
        }
    }
}

fn extract_plist_info(data_str: &str, meta: &mut HashMap<String, String>) {
    if let Some(start) = data_str.find("<key>CFBundleName</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let name = &data_str[start + value_start + 8..start + value_start + value_end];
                if !name.is_empty() && name.len() < 100 {
                    meta.insert("ProductName".into(), name.trim().to_string());
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>CFBundleDisplayName</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let name = &data_str[start + value_start + 8..start + value_start + value_end];
                if !name.is_empty() && name.len() < 100 {
                    if !meta.contains_key("ProductName") {
                        meta.insert("ProductName".into(), name.trim().to_string());
                    }
                    meta.insert("DisplayName".into(), name.trim().to_string());
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>CFBundleShortVersionString</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let version = &data_str[start + value_start + 8..start + value_start + value_end];
                if !version.is_empty() && version.len() < 50 {
                    meta.insert("ProductVersion".into(), version.trim().to_string());
                    meta.insert("FileVersion".into(), version.trim().to_string());
                }
            }
        }
    }
    
    if !meta.contains_key("ProductVersion") {
        if let Some(start) = data_str.find("<key>CFBundleVersion</key>") {
            if let Some(value_start) = data_str[start..].find("<string>") {
                if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                    let version = &data_str[start + value_start + 8..start + value_start + value_end];
                    if !version.is_empty() && version.len() < 50 {
                        meta.insert("ProductVersion".into(), version.trim().to_string());
                        meta.insert("FileVersion".into(), version.trim().to_string());
                        meta.insert("FileVersionNumber".into(), version.trim().to_string());
                    }
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>NSHumanReadableCopyright</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let copyright = &data_str[start + value_start + 8..start + value_start + value_end];
                if !copyright.is_empty() && copyright.len() < 200 {
                    meta.insert("LegalCopyright".into(), copyright.trim().to_string());
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>CFBundleGetInfoString</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let info = &data_str[start + value_start + 8..start + value_start + value_end];
                if !info.is_empty() && info.len() < 200 {
                    meta.insert("FileDescription".into(), info.trim().to_string());
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>LSApplicationCategoryType</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let category = &data_str[start + value_start + 8..start + value_start + value_end];
                if !category.is_empty() && category.len() < 100 {
                    let clean_category = category
                        .trim()
                        .split('.').next_back().unwrap_or(category)
                        .replace("-", " ")
                        .split_whitespace()
                        .map(capitalize_first)
                        .collect::<Vec<_>>()
                        .join(" ");
                    meta.insert("ApplicationCategory".into(), clean_category);
                }
            }
        }
    }
    
    if let Some(start) = data_str.find("<key>NSPrincipalClass</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let principal_class = &data_str[start + value_start + 8..start + value_start + value_end];
                if !principal_class.is_empty() && principal_class.len() < 100 {
                    meta.insert("PrincipalClass".into(), principal_class.trim().to_string());
                }
            }
        }
    }
}

fn extract_version_strings(data_str: &str, meta: &mut HashMap<String, String>) {
    if !meta.contains_key("ProductVersion") {
        if let Some(pos) = data_str.find("Version ") {
            let after_version = &data_str[pos + 8..];
            if let Some(end) = after_version.find(|c: char| !c.is_numeric() && c != '.') {
                let version_str = &after_version[..end];
                if !version_str.is_empty() && version_str.chars().all(|c| c.is_numeric() || c == '.') && version_str.contains('.') {
                    meta.insert("ProductVersion".into(), version_str.to_string());
                    meta.insert("FileVersion".into(), version_str.to_string());
                }
            }
        }
    }
}

fn extract_bundle_info(data_str: &str, meta: &mut HashMap<String, String>) {
    if let Some(start) = data_str.find("<key>CFBundleIdentifier</key>") {
        if let Some(value_start) = data_str[start..].find("<string>") {
            if let Some(value_end) = data_str[start + value_start..].find("</string>") {
                let bundle_id = &data_str[start + value_start + 8..start + value_start + value_end];
                if !bundle_id.is_empty() && bundle_id.len() < 200 {
                    meta.insert("BundleIdentifier".into(), bundle_id.trim().to_string());
                    
                    if !meta.contains_key("CompanyName") {
                        let parts: Vec<&str> = bundle_id.split('.').collect();
                        if parts.len() >= 2 {
                            let company = parts[1];
                            if !company.is_empty() && company.chars().all(|c| c.is_alphanumeric()) {
                                let company_name = capitalize_first(company);
                                meta.insert("CompanyName".into(), company_name.clone());
                                meta.insert("Manufacturer".into(), company_name);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn extract_developer_info(data_str: &str, meta: &mut HashMap<String, String>) {
    let company_patterns = [
        "Copyright",
        "Inc.",
        "Corporation",
        "Corp.",
        "LLC",
        "Ltd.",
        "Limited",
    ];
    
    for pattern in &company_patterns {
        if let Some(pos) = data_str.find(pattern) {
            let start = pos.saturating_sub(100).max(0);
            let end = (pos + 100).min(data_str.len());
            let context = &data_str[start..end];
            
            if pattern == &"Copyright" {
                if let Some(copy_pos) = context.find("Copyright") {
                    let after_copyright = &context[copy_pos + 9..];
                    let cleaned = after_copyright
                        .trim_start_matches(|c: char| c.is_numeric() || c == '©' || c == '(' || c == ')' || c == '-' || c.is_whitespace());
                    
                    if let Some(company_end) = cleaned.find(['\n', '\0', '.']) {
                        let company = &cleaned[..company_end];
                        if company.len() > 2 && company.len() < 100 && !meta.contains_key("CompanyName") {
                            meta.insert("CompanyName".into(), company.trim().to_string());
                            meta.insert("Publisher".into(), company.trim().to_string());
                        }
                    }
                }
            }
        }
    }
}

fn extract_app_names(data: &[u8], meta: &mut HashMap<String, String>) {
    let data_str = String::from_utf8_lossy(data);
    
    let skip_names = ["www", "html", "com", "http", "https", "ftp", "temp", "tmp", 
                      "test", "example", "demo", "data", "cache", "lib", "bin", "usr", "var",
                      "resources", "frameworks", "macos", "contents", "applications"];
    
    for match_pos in data_str.match_indices(".app") {
        let pos = match_pos.0;
        let start = pos.saturating_sub(100);
        
        let before = &data_str[start..pos];
        if let Some(last_slash) = before.rfind('/') {
            let app_name = &before[last_slash + 1..];
            let app_name_lower = app_name.to_lowercase();
            
            if app_name.len() > 2 && app_name.len() < 100 
                && app_name.chars().any(|c| c.is_alphabetic()) 
                && app_name.chars().filter(|c| c.is_alphabetic()).count() >= 3
                && app_name.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || c == '-' || c == '_')
                && !skip_names.contains(&app_name_lower.as_str())
                && !app_name_lower.starts_with("com.")
                && !app_name_lower.starts_with("org.") {
                meta.insert("ProductName".into(), app_name.trim().to_string());
                meta.insert("ApplicationBundle".into(), format!("{}.app", app_name.trim()));
                break;
            }
        }
    }
    
    if !meta.contains_key("ProductName") {
        let search_limit = 16384.min(data.len());
        let mut current_string = String::new();
        let mut valid_strings: Vec<String> = Vec::new();
        
        let skip_patterns = ["http", "www", "https", "ftp", "com.", "org.", ".app", "plist", "xml"];
        
        for &byte in &data[..search_limit] {
            if (32..=126).contains(&byte) {
                current_string.push(byte as char);
            } else if !current_string.is_empty() {
                if current_string.len() >= 5 && current_string.len() <= 100 {
                    let lower = current_string.to_lowercase();
                    
                    let has_installer_keyword = current_string.contains("Installer") || 
                                               current_string.contains("Setup");
                    
                    let is_clean_string = current_string.chars().filter(|c| c.is_alphabetic()).count() > 3 && 
                                         current_string.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace()).count() == current_string.len();
                    
                    let not_skipped = !skip_patterns.iter().any(|pat| lower.contains(pat));
                    
                    if (has_installer_keyword || is_clean_string) && not_skipped {
                        valid_strings.push(current_string.clone());
                    }
                }
                current_string.clear();
            }
        }
        
        if let Some(name) = valid_strings.iter().find(|s| s.contains("Installer") || s.contains("Setup")) {
            meta.insert("ProductName".into(), name.clone());
        } else if let Some(name) = valid_strings.first() {
            meta.insert("ProductName".into(), name.clone());
        }
    }
}

fn create_field_aliases(meta: &mut HashMap<String, String>) {
    if let Some(product_name) = meta.get("ProductName").cloned() {
        let sanitized = sanitize_string(&product_name);
        if !sanitized.is_empty() {
            meta.insert("ProductName".into(), sanitized.clone());
            if !meta.contains_key("ProgramName") {
                meta.insert("ProgramName".into(), sanitized.clone());
            }
            if !meta.contains_key("FileDescription") {
                meta.insert("FileDescription".into(), format!("{} Installer", sanitized));
            }
        }
    }
    
    if let Some(company) = meta.get("CompanyName").cloned() {
        if !meta.contains_key("Vendor") {
            meta.insert("Vendor".into(), company.clone());
        }
        if !meta.contains_key("Publisher") {
            meta.insert("Publisher".into(), company);
        }
    }
    
    if let Some(version) = meta.get("ProductVersion").cloned() {
        if !meta.contains_key("FileVersion") {
            meta.insert("FileVersion".into(), version.clone());
        }
        if !meta.contains_key("FileVersionNumber") {
            meta.insert("FileVersionNumber".into(), version.clone());
        }
        if !meta.contains_key("ProductVersionNumber") {
            meta.insert("ProductVersionNumber".into(), version);
        }
    }
    
    if !meta.contains_key("FileDescription") {
        meta.insert("FileDescription".into(), "Apple Disk Image".into());
    }
}

fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn sanitize_string(s: &str) -> String {
    s.chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}
