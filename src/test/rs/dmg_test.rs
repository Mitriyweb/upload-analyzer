use crate::dmg::{is_dmg_file, DMGAnalyzer};
use crate::FileAnalyzer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dmg_file() {
        // DMG with koly signature at the end
        let mut data = vec![0u8; 1024];
        let koly_offset = data.len() - 512;
        data[koly_offset..koly_offset + 4].copy_from_slice(b"koly");
        assert!(is_dmg_file(&data));

        // DMG with compression signature at the beginning AND koly at the end
        let mut data = vec![0u8; 1024];
        data[0..2].copy_from_slice(&[0x1F, 0x8B]); // Gzip
        let koly_offset = data.len() - 512;
        data[koly_offset..koly_offset + 4].copy_from_slice(b"koly");
        assert!(is_dmg_file(&data));

        let invalid_data = vec![0u8; 1024];
        assert!(!is_dmg_file(&invalid_data));
    }

    #[test]
    fn test_dmg_parsing_basic() {
        let mut data = vec![0u8; 1024];
        let koly_offset = data.len() - 512;
        data[koly_offset..koly_offset + 4].copy_from_slice(b"koly");
        // Version 4 bytes after koly
        let version = 4u32;
        data[koly_offset + 4..koly_offset + 8].copy_from_slice(&version.to_be_bytes());

        let result = DMGAnalyzer::parse_metadata(&data);
        assert!(result.is_ok());
        let meta = result.unwrap();
        assert_eq!(meta.get("Format").map(|s| s.as_str()), Some("DMG"));
        assert_eq!(meta.get("DMGVersion").map(|s| s.as_str()), Some("4"));
    }

    #[test]
    fn test_dmg_parsing_with_plist() {
        let plist_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>TestApp</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.testapp</string>
</dict>
</plist>"#;

        let mut data = vec![0u8; 2048];
        // Put plist somewhere in the middle
        let plist_pos = 100;
        data[plist_pos..plist_pos + plist_content.len()].copy_from_slice(plist_content.as_bytes());

        let koly_offset = data.len() - 512;
        data[koly_offset..koly_offset + 4].copy_from_slice(b"koly");

        let result = DMGAnalyzer::parse_metadata(&data);
        assert!(result.is_ok());
        let meta = result.unwrap();

        assert_eq!(meta.get("ProductName").map(|s| s.as_str()), Some("TestApp"));
        assert_eq!(meta.get("ProductVersion").map(|s| s.as_str()), Some("1.0.0"));
        assert_eq!(meta.get("BundleIdentifier").map(|s| s.as_str()), Some("com.example.testapp"));
        // Check alias/sanitized fields
        assert_eq!(meta.get("CompanyName").map(|s| s.as_str()), Some("Example"));
    }
}
