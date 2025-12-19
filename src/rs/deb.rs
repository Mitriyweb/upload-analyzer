use std::collections::HashMap;
use std::io::Read;
use ar::Archive;
use tar::Archive as TarArchive;
use flate2::read::GzDecoder;
use crate::{FileAnalyzer, MetadataResult};

pub struct DEBAnalyzer;

impl FileAnalyzer for DEBAnalyzer {
    fn get_file_info(data: &[u8]) -> HashMap<String, String> {
        let mut info = HashMap::new();
        info.insert("type".to_string(), "DEB (Debian Package)".to_string());
        info.insert("size".to_string(), data.len().to_string());
        info
    }

    fn parse_metadata(data: &[u8]) -> MetadataResult {
        let mut meta = HashMap::new();
        meta.insert("Format".into(), "DEB".into());

        let mut archive = Archive::new(data);
        let mut control_found = false;

        while let Some(entry_result) = archive.next_entry() {
            let entry = entry_result.map_err(|e| format!("Failed to read ar entry: {}", e))?;
            let header = entry.header();
            let name = std::str::from_utf8(header.identifier())
                .unwrap_or("")
                .trim_end_matches('/');

            if name.starts_with("control.tar") {
                control_found = true;

                // DEB control tarballs can be compressed with gzip (.gz), xz (.xz), etc.
                // We'll prioritize .gz for now as it's the most common for control.
                if name.ends_with(".gz") {
                    let decoder = GzDecoder::new(entry);
                    let mut tar = TarArchive::new(decoder);

                    for tar_entry_result in tar.entries().map_err(|e| format!("Failed to read tar entries: {}", e))? {
                        let mut tar_entry = tar_entry_result.map_err(|e| format!("Failed to read tar entry: {}", e))?;
                        let path = tar_entry.path().map_err(|e| format!("Failed to get tar path: {}", e))?;

                        if path.to_str() == Some("control") || path.to_str() == Some("./control") {
                            let mut control_content = String::new();
                            tar_entry.read_to_string(&mut control_content)
                                .map_err(|e| format!("Failed to read control file: {}", e))?;

                            parse_control_file(&control_content, &mut meta);
                            break;
                        }
                    }
                } else {
                    return Err(format!("Unsupported control archive compression: {}", name));
                }
                break;
            }
        }

        if !control_found {
            return Err("control.tar not found in DEB archive".to_string());
        }

        Ok(meta)
    }
}

fn parse_control_file(content: &str, meta: &mut HashMap<String, String>) {
    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();

            if !key.is_empty() && !value.is_empty() {
                // Map common DEB fields to our standard names if needed,
                // but for now we'll just keep them as is.
                meta.insert(key.to_string(), value.to_string());

                if key == "Architecture" {
                    meta.insert("Architecture".into(), value.to_string());
                }
            }
        }
    }
}

pub fn is_deb_file(data: &[u8]) -> bool {
    // DEB files start with !<arch>\n
    if data.len() < 8 || &data[0..8] != b"!<arch>\n" {
        return false;
    }

    // Also check for debian-binary member to be sure
    let mut archive = Archive::new(data);
    if let Some(Ok(entry)) = archive.next_entry() {
        let identifier = entry.header().identifier();
        return identifier == b"debian-binary" || identifier == b"debian-binary/   ";
    }

    false
}
