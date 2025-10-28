use pelite::pe64::{Pe as Pe64, PeFile as PeFile64};
use pelite::pe32::{Pe as Pe32, PeFile as PeFile32};
use goblin::pe::PE;
use std::collections::HashMap;
use crate::{msi, FileAnalyzer};

pub struct PEAnalyzer;

impl FileAnalyzer for PEAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "PE (Windows Executable)".to_string());
        info.insert("size".to_string(), data.len().to_string());
        info
    }
    
    fn parse_metadata(data: &[u8]) -> Result<HashMap<String, String>, String> {
        let pe = PE::parse(data).map_err(|e| format!("Failed to parse PE file: {}", e))?;
        parse_pe_metadata(data, &pe)
    }
}

fn parse_pe_metadata(buf: &[u8], pe: &PE) -> Result<HashMap<String, String>, String> {
    let mut meta = HashMap::new();
    
    meta.insert("Format".into(), "PE".into());
    
    detect_installer_type(buf, &mut meta);
    
    if pe.is_64 {
        meta.insert("Architecture".into(), "x64".into());
        extract_pe64_metadata(buf, &mut meta);
    } else {
        meta.insert("Architecture".into(), "x86".into());
        extract_pe32_metadata(buf, &mut meta);
    }
    
    Ok(meta)
}

fn detect_installer_type(buf: &[u8], meta: &mut HashMap<String, String>) {
    let buf_str = String::from_utf8_lossy(buf);
    
    if buf_str.contains("Inno Setup") || buf_str.contains("InnoSetupVersion") {
        meta.insert("InstallerType".into(), "Inno Setup".into());
    } else if buf_str.contains("Nullsoft Install System") || buf_str.contains("NSIS.Header") {
        meta.insert("InstallerType".into(), "NSIS (Nullsoft)".into());
    } else if buf_str.contains("Windows Installer") || buf_str.contains("InstallShield") {
        meta.insert("InstallerType".into(), "InstallShield".into());
    } else if buf_str.contains("WiX Toolset") || buf_str.contains("Windows Installer XML") {
        meta.insert("InstallerType".into(), "WiX Toolset".into());
    } else if buf_str.contains("Wise Installation System") {
        meta.insert("InstallerType".into(), "Wise Installer".into());
    } else if buf_str.contains("Setup Factory") {
        meta.insert("InstallerType".into(), "Setup Factory".into());
    } else if buf_str.contains("Smart Install Maker") {
        meta.insert("InstallerType".into(), "Smart Install Maker".into());
    }
    
    if let Some(pos) = find_bytes(buf, &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]) {
        meta.insert("EmbeddedMSI".into(), "true".into());
        meta.insert("MSIOffset".into(), pos.to_string());
        extract_embedded_msi_metadata(buf, pos, meta);
    }
    
    extract_signature_info(buf, meta);
}

fn extract_embedded_msi_metadata(buf: &[u8], msi_offset: usize, meta: &mut HashMap<String, String>) {
    if msi_offset >= buf.len() {
        return;
    }
    
    let msi_data = &buf[msi_offset..];
    
    if let Ok(msi_meta) = msi::MSIAnalyzer::parse_metadata(msi_data) {
        let msi_fields = [
            ("ProductName", "ProductName"),
            ("Manufacturer", "CompanyName"),
            ("Manufacturer", "Publisher"),
            ("ProductVersion", "Version"),
        ];
        
        for (msi_key, pe_key) in msi_fields.iter() {
            if let Some(value) = msi_meta.get(*msi_key) {
                if !meta.contains_key(*pe_key) {
                    meta.insert(format!("{}FromEmbeddedMSI", pe_key), value.clone());
                    meta.insert(pe_key.to_string(), format!("{} (from embedded MSI)", value));
                }
            }
        }
    }
}

fn extract_signature_info(buf: &[u8], meta: &mut HashMap<String, String>) {
    let patterns: &[(&[u8], usize)] = &[
        (b"O=" as &[u8], 2),
        (b"CN=" as &[u8], 3),
    ];
    
    for (pattern_bytes, pattern_len) in patterns {
        if let Some(pos) = find_bytes(buf, pattern_bytes) {
            let start = pos + pattern_len;
            if start >= buf.len() {
                continue;
            }
            
            let end = (start + 100).min(buf.len());
            let candidate = &buf[start..end];
            
            let mut text_end = 0;
            for (i, &byte) in candidate.iter().enumerate() {
                if byte == b',' || byte == 0 || byte < 32 || byte > 126 {
                    break;
                }
                text_end = i + 1;
            }
            
            if text_end > 0 && text_end >= 3 {
                if let Ok(name) = std::str::from_utf8(&candidate[..text_end]) {
                    let name = name.trim();
                    if name.len() >= 3 
                        && name.len() < 100 
                        && name.chars().any(|c| c.is_alphabetic())
                        && name.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '.' || *c == '-' || *c == ',' || *c == '&').count() == name.len()
                    {
                        meta.insert("SignedBy".into(), name.to_string());
                        if !meta.contains_key("CompanyName") {
                            meta.insert("CompanyName".into(), name.to_string());
                            meta.insert("Publisher".into(), name.to_string());
                        }
                        return;
                    }
                }
            }
        }
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn extract_pe32_metadata(buf: &[u8], meta: &mut HashMap<String, String>) {
    if let Ok(image) = PeFile32::from_bytes(&buf) {
        let header = image.file_header();
        
        meta.insert("Machine".into(), format!("0x{:04X}", header.Machine));
        meta.insert("NumberOfSections".into(), header.NumberOfSections.to_string());
        meta.insert("SizeOfOptionalHeader".into(), header.SizeOfOptionalHeader.to_string());
        meta.insert("Characteristics".into(), format!("0x{:04X}", header.Characteristics));
        meta.insert("PointerToSymbolTable".into(), header.PointerToSymbolTable.to_string());
        meta.insert("NumberOfSymbols".into(), header.NumberOfSymbols.to_string());
        
        if let Some(timestamp) = header.TimeDateStamp.to_string().parse::<i64>().ok() {
            if timestamp > 0 {
                meta.insert("Timestamp".into(), timestamp.to_string());
            }
        }
        
        let optional = image.optional_header();
        meta.insert("EntryPoint".into(), format!("0x{:08X}", optional.AddressOfEntryPoint));
        meta.insert("ImageBase".into(), format!("0x{:08X}", optional.ImageBase));
        meta.insert("SizeOfImage".into(), optional.SizeOfImage.to_string());
        meta.insert("Subsystem".into(), format!("{}", optional.Subsystem));
        meta.insert("DllCharacteristics".into(), format!("0x{:04X}", optional.DllCharacteristics));
        
        match image.resources() {
            Ok(rsrc) => {
                meta.insert("HasResources".into(), "true".into());
                match rsrc.version_info() {
                    Ok(ver) => {
                        meta.insert("HasVersionInfo".into(), "true".into());
                        
                        if let Some(fixed) = ver.fixed() {
                            let file_version = format!("{}.{}.{}.{}", 
                                fixed.dwFileVersion.Major,
                                fixed.dwFileVersion.Minor,
                                fixed.dwFileVersion.Patch,
                                fixed.dwFileVersion.Build
                            );
                            meta.insert("FileVersionNumber".into(), file_version);
                            
                            let product_version = format!("{}.{}.{}.{}", 
                                fixed.dwProductVersion.Major,
                                fixed.dwProductVersion.Minor,
                                fixed.dwProductVersion.Patch,
                                fixed.dwProductVersion.Build
                            );
                            meta.insert("ProductVersionNumber".into(), product_version);
                            
                            meta.insert("FileFlags".into(), format!("0x{:08X}", fixed.dwFileFlags));
                            meta.insert("FileOS".into(), format!("0x{:08X}", fixed.dwFileOS));
                            meta.insert("FileType".into(), format!("0x{:08X}", fixed.dwFileType));
                        }
                        
                        let translations = ver.translation();
                        meta.insert("TranslationCount".into(), translations.len().to_string());
                        
                        let mut all_strings = HashMap::new();
                        let mut strings_per_lang: Vec<usize> = Vec::new();
                        
                        for (idx, lang) in translations.iter().enumerate() {
                            meta.insert(format!("Translation_{}", idx), format!("{:?}", lang));
                            
                            let mut count = 0;
                            ver.strings(*lang, |key, value| {
                                count += 1;
                                meta.insert(format!("Debug_{}_{}", idx, key), value.to_string());
                                if !value.is_empty() {
                                    all_strings.insert(key.to_string(), value.to_string());
                                }
                            });
                            
                            strings_per_lang.push(count);
                            meta.insert(format!("StringsInTranslation_{}", idx), count.to_string());
                        }
                        
                        meta.insert("TotalCallbackCalls".into(), strings_per_lang.iter().sum::<usize>().to_string());
                        
                        meta.insert("StringsCount".into(), all_strings.len().to_string());
                        
                        if !all_strings.is_empty() {
                            for (key, value) in all_strings.iter() {
                                meta.insert(key.clone(), value.clone());
                                
                                match key.as_str() {
                                    "FileDescription" => { meta.insert("ProgramName".into(), value.clone()); }
                                    "CompanyName" => { 
                                        meta.insert("Vendor".into(), value.clone());
                                        meta.insert("Publisher".into(), value.clone());
                                    }
                                    "ProductVersion" => { meta.insert("Version".into(), value.clone()); }
                                    _ => {}
                                }
                            }
                        } else {
                            meta.insert("NoStringsFound".into(), "true".into());
                            if let Some(company) = meta.get("CompanyName").cloned() {
                                if meta.contains_key("SignedBy") && !company.contains("from digital signature") {
                                    meta.insert("CompanyName".into(), format!("{} (from digital signature)", company));
                                    if let Some(publisher) = meta.get("Publisher").cloned() {
                                        meta.insert("Publisher".into(), format!("{} (from digital signature)", publisher));
                                    }
                                    if let Some(vendor) = meta.get("Vendor").cloned() {
                                        meta.insert("Vendor".into(), format!("{} (from digital signature)", vendor));
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        meta.insert("VersionInfoError".into(), format!("{:?}", e));
                    }
                }
            }
            Err(e) => {
                meta.insert("ResourcesError".into(), format!("{:?}", e));
            }
        }
    }
}

fn extract_pe64_metadata(buf: &[u8], meta: &mut HashMap<String, String>) {
    if let Ok(image) = PeFile64::from_bytes(&buf) {
        let header = image.file_header();
        
        meta.insert("Machine".into(), format!("0x{:04X}", header.Machine));
        meta.insert("NumberOfSections".into(), header.NumberOfSections.to_string());
        meta.insert("SizeOfOptionalHeader".into(), header.SizeOfOptionalHeader.to_string());
        meta.insert("Characteristics".into(), format!("0x{:04X}", header.Characteristics));
        meta.insert("PointerToSymbolTable".into(), header.PointerToSymbolTable.to_string());
        meta.insert("NumberOfSymbols".into(), header.NumberOfSymbols.to_string());
        
        if let Some(timestamp) = header.TimeDateStamp.to_string().parse::<i64>().ok() {
            if timestamp > 0 {
                meta.insert("Timestamp".into(), timestamp.to_string());
            }
        }
        
        let optional = image.optional_header();
        meta.insert("EntryPoint".into(), format!("0x{:08X}", optional.AddressOfEntryPoint));
        meta.insert("ImageBase".into(), format!("0x{:016X}", optional.ImageBase));
        meta.insert("SizeOfImage".into(), optional.SizeOfImage.to_string());
        meta.insert("Subsystem".into(), format!("{}", optional.Subsystem));
        meta.insert("DllCharacteristics".into(), format!("0x{:04X}", optional.DllCharacteristics));
        
        match image.resources() {
            Ok(rsrc) => {
                meta.insert("HasResources".into(), "true".into());
                match rsrc.version_info() {
                    Ok(ver) => {
                        meta.insert("HasVersionInfo".into(), "true".into());
                        
                        if let Some(fixed) = ver.fixed() {
                            let file_version = format!("{}.{}.{}.{}", 
                                fixed.dwFileVersion.Major,
                                fixed.dwFileVersion.Minor,
                                fixed.dwFileVersion.Patch,
                                fixed.dwFileVersion.Build
                            );
                            meta.insert("FileVersionNumber".into(), file_version);
                            
                            let product_version = format!("{}.{}.{}.{}", 
                                fixed.dwProductVersion.Major,
                                fixed.dwProductVersion.Minor,
                                fixed.dwProductVersion.Patch,
                                fixed.dwProductVersion.Build
                            );
                            meta.insert("ProductVersionNumber".into(), product_version);
                            
                            meta.insert("FileFlags".into(), format!("0x{:08X}", fixed.dwFileFlags));
                            meta.insert("FileOS".into(), format!("0x{:08X}", fixed.dwFileOS));
                            meta.insert("FileType".into(), format!("0x{:08X}", fixed.dwFileType));
                        }
                        
                        let translations = ver.translation();
                        meta.insert("TranslationCount".into(), translations.len().to_string());
                        
                        let mut all_strings = HashMap::new();
                        let mut strings_per_lang: Vec<usize> = Vec::new();
                        
                        for (idx, lang) in translations.iter().enumerate() {
                            meta.insert(format!("Translation_{}", idx), format!("{:?}", lang));
                            
                            let mut count = 0;
                            ver.strings(*lang, |key, value| {
                                count += 1;
                                meta.insert(format!("Debug_{}_{}", idx, key), value.to_string());
                                if !value.is_empty() {
                                    all_strings.insert(key.to_string(), value.to_string());
                                }
                            });
                            
                            strings_per_lang.push(count);
                            meta.insert(format!("StringsInTranslation_{}", idx), count.to_string());
                        }
                        
                        meta.insert("TotalCallbackCalls".into(), strings_per_lang.iter().sum::<usize>().to_string());
                        
                        meta.insert("StringsCount".into(), all_strings.len().to_string());
                        
                        if !all_strings.is_empty() {
                            for (key, value) in all_strings.iter() {
                                meta.insert(key.clone(), value.clone());
                                
                                match key.as_str() {
                                    "FileDescription" => { meta.insert("ProgramName".into(), value.clone()); }
                                    "CompanyName" => { 
                                        meta.insert("Vendor".into(), value.clone());
                                        meta.insert("Publisher".into(), value.clone());
                                    }
                                    "ProductVersion" => { meta.insert("Version".into(), value.clone()); }
                                    _ => {}
                                }
                            }
                        } else {
                            meta.insert("NoStringsFound".into(), "true".into());
                            if let Some(company) = meta.get("CompanyName").cloned() {
                                if meta.contains_key("SignedBy") && !company.contains("from digital signature") {
                                    meta.insert("CompanyName".into(), format!("{} (from digital signature)", company));
                                    if let Some(publisher) = meta.get("Publisher").cloned() {
                                        meta.insert("Publisher".into(), format!("{} (from digital signature)", publisher));
                                    }
                                    if let Some(vendor) = meta.get("Vendor").cloned() {
                                        meta.insert("Vendor".into(), format!("{} (from digital signature)", vendor));
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        meta.insert("VersionInfoError".into(), format!("{:?}", e));
                    }
                }
            }
            Err(e) => {
                meta.insert("ResourcesError".into(), format!("{:?}", e));
            }
        }
    }
}
