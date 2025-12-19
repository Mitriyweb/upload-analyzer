use pelite::pe64::{Pe as Pe64, PeFile as PeFile64};
use pelite::pe32::{Pe as Pe32, PeFile as PeFile32};
use goblin::pe::PE;
use std::collections::HashMap;
use crate::{msi, FileAnalyzer, MetadataResult};

// Constants for magic numbers and patterns
const MSI_SIGNATURE: &[u8] = &[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

// Installer type patterns
const PATTERN_INNO_SETUP: &[u8] = b"Inno Setup";
const PATTERN_INNO_VERSION: &[u8] = b"InnoSetupVersion";
const PATTERN_NSIS: &[u8] = b"Nullsoft Install System";
const PATTERN_NSIS_HEADER: &[u8] = b"NSIS.Header";
const PATTERN_WINDOWS_INSTALLER: &[u8] = b"Windows Installer";
const PATTERN_INSTALLSHIELD: &[u8] = b"InstallShield";
const PATTERN_WIX: &[u8] = b"WiX Toolset";
const PATTERN_WIX_XML: &[u8] = b"Windows Installer XML";
const PATTERN_WISE: &[u8] = b"Wise Installation System";
const PATTERN_SETUP_FACTORY: &[u8] = b"Setup Factory";
const PATTERN_SMART_INSTALL: &[u8] = b"Smart Install Maker";

pub struct PEAnalyzer;

impl FileAnalyzer for PEAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "PE (Windows Executable)".to_string());
        info.insert("size".to_string(), data.len().to_string());
        info
    }

    fn parse_metadata(data: &[u8]) -> MetadataResult {
        let pe = PE::parse(data).map_err(|e| format!("Failed to parse PE file: {}", e))?;
        parse_pe_metadata(data, &pe)
    }
}

fn parse_pe_metadata(buf: &[u8], pe: &PE) -> MetadataResult {
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
    // Helper to check if a pattern exists in buffer
    let contains_pattern = |pattern: &[u8]| -> bool {
        find_bytes(buf, pattern).is_some()
    };

    // Check installer patterns efficiently without converting to String
    if contains_pattern(PATTERN_INNO_SETUP) || contains_pattern(PATTERN_INNO_VERSION) {
        meta.insert("InstallerType".to_string(), "Inno Setup".to_string());
    } else if contains_pattern(PATTERN_NSIS) || contains_pattern(PATTERN_NSIS_HEADER) {
        meta.insert("InstallerType".to_string(), "NSIS (Nullsoft)".to_string());
    } else if contains_pattern(PATTERN_WINDOWS_INSTALLER) || contains_pattern(PATTERN_INSTALLSHIELD) {
        meta.insert("InstallerType".to_string(), "InstallShield".to_string());
    } else if contains_pattern(PATTERN_WIX) || contains_pattern(PATTERN_WIX_XML) {
        meta.insert("InstallerType".to_string(), "WiX Toolset".to_string());
    } else if contains_pattern(PATTERN_WISE) {
        meta.insert("InstallerType".to_string(), "Wise Installer".to_string());
    } else if contains_pattern(PATTERN_SETUP_FACTORY) {
        meta.insert("InstallerType".to_string(), "Setup Factory".to_string());
    } else if contains_pattern(PATTERN_SMART_INSTALL) {
        meta.insert("InstallerType".to_string(), "Smart Install Maker".to_string());
    }

    // Check for embedded MSI
    if let Some(pos) = find_bytes(buf, MSI_SIGNATURE) {
        meta.insert("EmbeddedMSI".to_string(), "true".to_string());
        meta.insert("MSIOffset".to_string(), pos.to_string());
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
            ("Manufacturer", "Manufacturer"),
            ("ProductVersion", "ProductVersion"),
        ];

        for (msi_key, pe_key) in msi_fields.iter() {
            if let Some(value) = msi_meta.get(*msi_key) {
                if !meta.contains_key(*pe_key) {
                    meta.insert(format!("{}FromEmbeddedMSI", pe_key), value.clone());
                    meta.insert((*pe_key).to_string(), format!("{} (from embedded MSI)", value));
                }
            }
        }
    }
}

fn extract_signature_info(buf: &[u8], meta: &mut HashMap<String, String>) {
    let patterns = [
        (b"O=" as &[u8], 2),
        (b"CN=" as &[u8], 3),
    ];

    for (pattern_bytes, pattern_len) in patterns.iter() {
        if let Some(pos) = find_bytes(buf, pattern_bytes) {
            let start = pos + pattern_len;
            if start >= buf.len() {
                continue;
            }

            let end = (start + 100).min(buf.len());
            let candidate = &buf[start..end];

            let mut text_end = 0;
            for (i, &byte) in candidate.iter().enumerate() {
                if byte == b',' || byte == 0 || !(32..=126).contains(&byte) {
                    break;
                }
                text_end = i + 1;
            }

            if text_end >= 3 {
                if let Ok(name) = std::str::from_utf8(&candidate[..text_end]) {
                    let name = name.trim();
                    if name.len() >= 3
                        && name.len() < 100
                        && name.chars().any(|c| c.is_alphabetic())
                        && name.chars().filter(|c| c.is_alphanumeric() || c.is_whitespace() || *c == '.' || *c == '-' || *c == ',' || *c == '&').count() == name.len()
                    {
                        meta.insert("SignedBy".into(), name.to_string());
                        return;
                    }
                }
            }
        }
    }
}

#[inline]
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

        if let Ok(timestamp) = header.TimeDateStamp.to_string().parse::<i64>() {
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
                            }
                        } else {
                            meta.insert("NoStringsFound".into(), "true".into());
                            if let Some(company) = meta.get("CompanyName").cloned() {
                                if meta.contains_key("SignedBy") && !company.contains("from digital signature") {
                                    meta.insert("CompanyName".into(), format!("{} (from digital signature)", company));
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

        if let Ok(timestamp) = header.TimeDateStamp.to_string().parse::<i64>() {
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
                            }
                        } else {
                            meta.insert("NoStringsFound".into(), "true".into());
                            if let Some(company) = meta.get("CompanyName").cloned() {
                                if meta.contains_key("SignedBy") && !company.contains("from digital signature") {
                                    meta.insert("CompanyName".into(), format!("{} (from digital signature)", company));
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
