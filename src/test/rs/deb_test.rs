use std::io::Write;
use crate::deb::{is_deb_file, DEBAnalyzer};
use crate::FileAnalyzer;
use flate2::write::GzEncoder;
use flate2::Compression;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_deb_file() {
        // Minimal valid ar archive with debian-binary
        let mut data = Vec::new();
        data.extend_from_slice(b"!<arch>\n");
        // Header for debian-binary: name (16), date (12), uid (6), gid (6), mode (8), size (10), magic (2)
        data.extend_from_slice(b"debian-binary   0           0     0     644     4         `\n");
        data.extend_from_slice(b"2.0\n");

        assert!(is_deb_file(&data));

        let invalid_data = b"not a deb file";
        assert!(!is_deb_file(invalid_data));
    }

    #[test]
    fn test_deb_parsing() {
        let mut data = Vec::new();
        data.extend_from_slice(b"!<arch>\n");

        // 1. debian-binary
        data.extend_from_slice(b"debian-binary   0           0     0     644     4         `\n");
        data.extend_from_slice(b"2.0\n");

        // 2. control.tar.gz
        let mut tar_data = Vec::new();
        {
            let mut tar_builder = tar::Builder::new(&mut tar_data);
            let control_content = b"Package: test-package\nVersion: 1.2.3\nArchitecture: amd64\n";
            let mut header = tar::Header::new_gnu();
            header.set_path("control").unwrap();
            header.set_size(control_content.len() as u64);
            header.set_cksum();
            tar_builder.append(&header, &control_content[..]).unwrap();
            tar_builder.finish().unwrap();
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&tar_data).unwrap();
        let compressed_tar = encoder.finish().unwrap();

        // Ar header for control.tar.gz
        let name = "control.tar.gz  ";
        let size = format!("{:<10}", compressed_tar.len());
        let ar_header = format!("{}0           0     0     644     {}`\n", name, size);
        data.extend_from_slice(ar_header.as_bytes());
        data.extend_from_slice(&compressed_tar);
        if compressed_tar.len() % 2 != 0 {
            data.push(b'\n'); // Padding
        }

        let result = DEBAnalyzer::parse_metadata(&data);
        assert!(result.is_ok(), "Failed to parse DEB: {:?}", result.err());

        let meta = result.unwrap();
        assert_eq!(meta.get("Format").map(|s| s.as_str()), Some("DEB"));
        assert_eq!(meta.get("Package").map(|s| s.as_str()), Some("test-package"));
        assert_eq!(meta.get("Version").map(|s| s.as_str()), Some("1.2.3"));
        assert_eq!(meta.get("Architecture").map(|s| s.as_str()), Some("amd64"));
    }
}
