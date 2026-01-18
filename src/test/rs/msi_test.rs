use crate::msi::{is_msi_file, MSIAnalyzer};
use crate::FileAnalyzer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_msi_file() {
        let mut data = vec![0u8; 8];
        data[0..8].copy_from_slice(&[0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]);
        assert!(is_msi_file(&data));

        let invalid_data = vec![0u8; 8];
        assert!(!is_msi_file(&invalid_data));
    }

    #[test]
    fn test_msi_parsing_invalid() {
        // MSI parsing of random data should fail or return basic info if heuristics fail
        let data = vec![0u8; 100];
        let result = MSIAnalyzer::parse_metadata(&data);
        assert!(result.is_ok()); // It has fallbacks
        let meta = result.unwrap();
        assert_eq!(meta.get("Format").map(|s| s.as_str()), Some("MSI"));
    }
}
