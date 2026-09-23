//! Recovery algorithm entry points.

use crate::{FileEntry, Result};

/// A known file format signature used by carving.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileSignature {
    /// Stable format identifier.
    pub name: &'static str,
    /// Header bytes.
    pub header: &'static [u8],
    /// Suggested MIME type.
    pub mime_type: &'static str,
}

/// Small, conservative signature catalog for the first carving pass.
pub const SIGNATURES: &[FileSignature] = &[
    FileSignature {
        name: "jpeg",
        header: &[0xFF, 0xD8, 0xFF],
        mime_type: "image/jpeg",
    },
    FileSignature {
        name: "png",
        header: &[0x89, 0x50, 0x4E, 0x47],
        mime_type: "image/png",
    },
    FileSignature {
        name: "pdf",
        header: b"%PDF-",
        mime_type: "application/pdf",
    },
    FileSignature {
        name: "zip",
        header: &[0x50, 0x4B, 0x03, 0x04],
        mime_type: "application/zip",
    },
    FileSignature {
        name: "gzip",
        header: &[0x1F, 0x8B],
        mime_type: "application/gzip",
    },
    FileSignature {
        name: "sqlite",
        header: b"SQLite format 3\0",
        mime_type: "application/vnd.sqlite3",
    },
];

/// A progress event emitted by long-running recovery operations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum ProgressEvent {
    /// Scan started.
    ScanStart { device: String, mode: String },
    /// Percentage update.
    Progress { percentage: f32, bytes_scanned: u64 },
    /// A file was recovered.
    FileRecovered { file: FileEntry },
    /// Scan completed.
    ScanComplete { total_files: u64 },
}

/// Find a known byte signature in a buffer.
pub fn find_signature(data: &[u8], signature: &[u8]) -> Option<usize> {
    if signature.is_empty() {
        return Some(0);
    }
    data.windows(signature.len())
        .position(|window| window == signature)
}

/// Validate a confidence value used by recovery results.
pub fn validate_confidence(confidence: f32) -> Result<()> {
    if confidence.is_finite() && (0.0..=1.0).contains(&confidence) {
        Ok(())
    } else {
        Err(crate::RecoveryError::LowConfidence { confidence })
    }

    /// Return all known signatures found in a buffer.
    pub fn detect_signatures(data: &[u8]) -> Vec<(FileSignature, usize)> {
        SIGNATURES
            .iter()
            .filter_map(|signature| {
                find_signature(data, signature.header).map(|offset| (*signature, offset))
            })
            .collect()
    }

    /// Scan a buffer and return one catalog entry per detected header.
    pub fn carve_headers(data: &[u8]) -> Vec<FileEntry> {
        detect_signatures(data)
            .into_iter()
            .enumerate()
            .map(|(index, (signature, offset))| {
                let mut entry = FileEntry::new(
                    std::path::PathBuf::from(format!("carved_{index:05}.{}", signature.name)),
                    (data.len() - offset) as u64,
                    crate::FilesystemType::Unknown,
                    offset as u64,
                );
                entry.mime_type = signature.mime_type.to_string();
                entry.recovery_status = crate::models::RecoveryStatus::Carved;
                entry.recovery_confidence = 0.75;
                entry.recovery_details = Some(crate::models::RecoveryDetails {
                    method: "signature_header".to_string(),
                    sectors_recovered: 0,
                    sectors_missing: 0,
                    confidence: 0.75,
                });
                entry
            })
            .collect()
    }
}
