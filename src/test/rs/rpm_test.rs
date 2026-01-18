use crate::rpm::{is_rpm_file, RPMAnalyzer};
use crate::FileAnalyzer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rpm_file() {
        let mut data = vec![0; 100];
        data[0..4].copy_from_slice(&[0xED, 0xAB, 0xEE, 0xDB]);
        assert!(is_rpm_file(&data));

        let invalid_data = vec![0; 100];
        assert!(!is_rpm_file(&invalid_data));
    }

    #[test]
    fn test_rpm_parsing() {
        // Construct a minimal valid RPM file structure
        let mut data = vec![0u8; 96]; // Lead
        data[0..4].copy_from_slice(&[0xED, 0xAB, 0xEE, 0xDB]);

        // Signature Header (empty but valid)
        data.extend_from_slice(&[0x8E, 0xAD, 0xE8, 0x01]); // Magic
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Reserved
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Index count
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Store size

        // Main Header
        data.extend_from_slice(&[0x8E, 0xAD, 0xE8, 0x01]); // Magic
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Reserved

        let index_count = 2u32;
        let store_data = b"test-package\x001.0.0\0";
        let store_size = store_data.len() as u32;

        data.extend_from_slice(&index_count.to_be_bytes());
        data.extend_from_slice(&store_size.to_be_bytes());

        // Index Entry 1: NAME (Tag 1000, Type 6 (String), Offset 0, Count 1)
        data.extend_from_slice(&1000u32.to_be_bytes());
        data.extend_from_slice(&6u32.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&1u32.to_be_bytes());

        // Index Entry 2: VERSION (Tag 1001, Type 6 (String), Offset 13, Count 1)
        data.extend_from_slice(&1001u32.to_be_bytes());
        data.extend_from_slice(&6u32.to_be_bytes());
        data.extend_from_slice(&13u32.to_be_bytes());
        data.extend_from_slice(&1u32.to_be_bytes());

        // Store
        data.extend_from_slice(store_data);

        let result = RPMAnalyzer::parse_metadata(&data);
        assert!(result.is_ok(), "Failed to parse RPM: {:?}", result.err());

        let meta = result.expect("Should be Ok");
        assert_eq!(meta.get("Format").map(|s| s.as_str()), Some("RPM"));
        assert_eq!(meta.get("ProductName").map(|s| s.as_str()), Some("test-package"));
        assert_eq!(meta.get("ProductVersion").map(|s| s.as_str()), Some("1.0.0"));
    }

    #[test]
    fn test_rpm_invalid_header() {
        let mut data = vec![0u8; 96];
        data[0..4].copy_from_slice(&[0xED, 0xAB, 0xEE, 0xDB]);
        data.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // Invalid magic

        let result = RPMAnalyzer::parse_metadata(&data);
        assert!(result.is_err());
    }
}
