use goblin::mach::Mach;
use std::collections::HashMap;

pub fn get_file_info(data: &[u8]) -> HashMap<String, String> {
    let mut info = HashMap::new();
    info.insert("type".to_string(), "Mach-O (macOS)".to_string());
    info.insert("size".to_string(), data.len().to_string());
    info
}

pub fn parse_macho_metadata(mach: &Mach) -> Result<HashMap<String, String>, String> {
    let mut meta = HashMap::new();
    meta.insert("Format".into(), "Mach-O".into());
    
    match mach {
        goblin::mach::Mach::Binary(macho) => {
            for segment in &macho.segments {
                if let Ok(name) = segment.name() {
                    if name == "__TEXT" {
                        meta.insert("Segment".into(), "__TEXT found".into());
                        break;
                    }
                }
            }
        }
        _ => {}
    }
    
    Ok(meta)
}
