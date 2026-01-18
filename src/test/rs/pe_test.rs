use crate::pe::{detect_installer_type, extract_signature_info, PEAnalyzer};
use crate::FileAnalyzer;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_get_file_info() {
        let data = vec![0u8; 100];
        let info = PEAnalyzer::get_file_info(&data);
        assert_eq!(info.get("Format").map(|s| s.as_str()), Some("PE"));
    }

    #[test]
    fn test_installer_detection() {
        let mut meta = HashMap::new();

        // Inno Setup
        let data = b"Some data with Inno Setup here";
        detect_installer_type(data, &mut meta);
        assert_eq!(meta.get("InstallerType").map(|s| s.as_str()), Some("Inno Setup"));

        // NSIS
        meta.clear();
        let data = b"Nullsoft Install System is great";
        detect_installer_type(data, &mut meta);
        assert_eq!(meta.get("InstallerType").map(|s| s.as_str()), Some("NSIS (Nullsoft)"));
    }

    #[test]
    fn test_signature_extraction() {
        let mut meta = HashMap::new();

        // Signed by CN=Test Certificate
        let data = b"prefix CN=Test Certificate,suffix";
        extract_signature_info(data, &mut meta);
        assert_eq!(meta.get("SignedBy").map(|s| s.as_str()), Some("Test Certificate"));

        // Signed by O=Test Organization
        meta.clear();
        let data = b"some data O=Test Organization, more data";
        extract_signature_info(data, &mut meta);
        assert_eq!(meta.get("SignedBy").map(|s| s.as_str()), Some("Test Organization"));
    }

    #[test]
    fn test_signature_extraction_invalid() {
        let mut meta = HashMap::new();

        // Name too short after invalid char
        let data = b"CN=Te\x01Certificate";
        extract_signature_info(data, &mut meta);
        assert!(!meta.contains_key("SignedBy"));
    }
}
