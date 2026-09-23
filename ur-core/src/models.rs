//! Core data model for UniversalRecovery
//!
//! Defines JSON-serializable structs for representing recovered files,
//! filesystem information, and recovery sessions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Supported filesystem types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilesystemType {
    #[serde(rename = "ext4")]
    Ext4,
    #[serde(rename = "ext3")]
    Ext3,
    #[serde(rename = "ntfs")]
    Ntfs,
    #[serde(rename = "fat32")]
    Fat32,
    #[serde(rename = "exfat")]
    ExFat,
    #[serde(rename = "iso9660")]
    Iso9660,
    #[serde(rename = "btrfs")]
    Btrfs,
    #[serde(rename = "xfs")]
    Xfs,
    #[serde(rename = "unknown")]
    Unknown,
}

impl std::fmt::Display for FilesystemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilesystemType::Ext4 => write!(f, "ext4"),
            FilesystemType::Ext3 => write!(f, "ext3"),
            FilesystemType::Ntfs => write!(f, "NTFS"),
            FilesystemType::Fat32 => write!(f, "FAT32"),
            FilesystemType::ExFat => write!(f, "exFAT"),
            FilesystemType::Iso9660 => write!(f, "ISO 9660"),
            FilesystemType::Btrfs => write!(f, "Btrfs"),
            FilesystemType::Xfs => write!(f, "XFS"),
            FilesystemType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Recovery status of a file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStatus {
    /// File recovered from inode/metadata
    Recovered,
    /// File recovered from sector carving
    Carved,
    /// File partially recovered (some blocks missing)
    Partial,
    /// File recovery attempted but failed
    Failed,
    /// File marked as deleted but recoverable
    Deleted,
}

/// Details about file recovery method
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryDetails {
    /// Recovery method used (e.g., "inode_chain", "carving", "mft_scan")
    pub method: String,
    /// Number of sectors recovered
    pub sectors_recovered: u64,
    /// Number of sectors missing/unrecoverable
    pub sectors_missing: u64,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f32,
}

/// Represents a recovered file from filesystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileEntry {
    /// Unique file identifier (UUID)
    pub file_id: String,
    /// Relative path from filesystem root
    pub relative_path: PathBuf,
    /// File size in bytes
    pub size_bytes: u64,
    /// MIME type (e.g., "image/jpeg")
    pub mime_type: String,
    /// Filesystem containing this file
    pub filesystem: FilesystemType,
    /// Inode number (filesystem-specific)
    pub inode: u64,
    /// Recovery status
    pub recovery_status: RecoveryStatus,
    /// Confidence score (0.0 = no confidence, 1.0 = certainty)
    pub recovery_confidence: f32,
    /// SHA256 checksum of recovered file
    pub checksum_sha256: String,
    /// Creation timestamp (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Modification timestamp (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<DateTime<Utc>>,
    /// Detailed recovery information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_details: Option<RecoveryDetails>,
}

impl FileEntry {
    /// Create a new file entry
    pub fn new(
        relative_path: PathBuf,
        size_bytes: u64,
        filesystem: FilesystemType,
        inode: u64,
    ) -> Self {
        Self {
            file_id: Uuid::new_v4().to_string(),
            relative_path,
            size_bytes,
            mime_type: "application/octet-stream".to_string(),
            filesystem,
            inode,
            recovery_status: RecoveryStatus::Recovered,
            recovery_confidence: 0.5,
            checksum_sha256: String::new(),
            created_at: None,
            modified_at: None,
            recovery_details: None,
        }
    }
}

/// Generic inode representation across filesystems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inode {
    /// Inode number
    pub number: u64,
    /// File size in bytes
    pub size: u64,
    /// POSIX permissions (0o755, etc.)
    pub permissions: u32,
    /// Owner user ID
    pub owner_uid: u32,
    /// Owner group ID
    pub owner_gid: u32,
    /// Creation/birth time
    pub created_time: DateTime<Utc>,
    /// Access time
    pub accessed_time: DateTime<Utc>,
    /// Modification time
    pub modified_time: DateTime<Utc>,
    /// Block addresses (direct blocks first, then indirect)
    pub blocks: Vec<u64>,
    /// Link count (for hardlinks)
    pub link_count: u32,
    /// Deletion time (0 if not deleted)
    pub deletion_time: Option<DateTime<Utc>>,
}

impl Inode {
    /// Check if inode is valid
    pub fn is_valid(&self) -> bool {
        // Size 0 shouldn't have blocks
        if self.size == 0 && !self.blocks.is_empty() {
            return false;
        }
        // Link count must be > 0 for live inodes
        if self.link_count == 0 && self.deletion_time.is_none() {
            return false;
        }
        true
    }

    /// Check if inode represents a deleted file
    pub fn is_deleted(&self) -> bool {
        self.link_count == 0 || self.deletion_time.is_some()
    }
}

/// Detected filesystem on device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFilesystem {
    /// Filesystem type
    pub filesystem_type: FilesystemType,
    /// Starting offset (bytes)
    pub start_offset: u64,
    /// Total capacity (bytes)
    pub capacity: u64,
    /// Used space (bytes)
    pub used_space: u64,
    /// Filesystem label/name
    pub label: String,
    /// Total number of files found
    pub total_files: u64,
}

/// Session info for recovery tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    /// Unique session ID
    pub session_id: String,
    /// Device scanned (e.g., "/dev/sda1")
    pub device: String,
    /// Scan start time
    pub start_time: DateTime<Utc>,
    /// Scan end time (None if ongoing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    /// Total bytes scanned
    pub total_bytes_scanned: u64,
    /// Scan mode ("full", "quick", "carving_only")
    pub scan_mode: String,
}

/// Recovery statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    /// Total files recovered
    pub total_files: u64,
    /// Files with high confidence (>= 0.9)
    pub high_confidence: u64,
    /// Files with medium confidence (0.7 - 0.9)
    pub medium_confidence: u64,
    /// Files with low confidence (< 0.7)
    pub low_confidence: u64,
    /// Total bytes recovered
    pub total_bytes: u64,
    /// Total scan time (seconds)
    pub scan_duration_sec: u64,
    /// Average throughput (MB/s)
    pub throughput_mb_s: f32,
}

/// Master recovery catalog and index
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryIndex {
    /// Catalog version
    pub catalog_version: String,
    /// Session metadata
    pub recovery_session: SessionInfo,
    /// Recovery statistics
    pub statistics: Statistics,
    /// Detected filesystems
    pub filesystems: Vec<DetectedFilesystem>,
    /// All recovered files
    pub file_entries: Vec<FileEntry>,
    /// Errors encountered during recovery
    #[serde(default)]
    pub errors: Vec<String>,
}

impl RecoveryIndex {
    /// Create new recovery index
    pub fn new(device: String, scan_mode: String) -> Self {
        Self {
            catalog_version: "1.0".to_string(),
            recovery_session: SessionInfo {
                session_id: Uuid::new_v4().to_string(),
                device,
                start_time: Utc::now(),
                end_time: None,
                total_bytes_scanned: 0,
                scan_mode,
            },
            statistics: Statistics::default(),
            filesystems: Vec::new(),
            file_entries: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Finish recovery session
    pub fn finish(&mut self) {
        self.recovery_session.end_time = Some(Utc::now());

        // Recalculate statistics
        self.statistics.total_files = self.file_entries.len() as u64;
        self.statistics.total_bytes = self.file_entries.iter().map(|f| f.size_bytes).sum();

        let high = self
            .file_entries
            .iter()
            .filter(|f| f.recovery_confidence >= 0.9)
            .count() as u64;
        let medium = self
            .file_entries
            .iter()
            .filter(|f| f.recovery_confidence >= 0.7 && f.recovery_confidence < 0.9)
            .count() as u64;
        let low = self
            .file_entries
            .iter()
            .filter(|f| f.recovery_confidence < 0.7)
            .count() as u64;

        self.statistics.high_confidence = high;
        self.statistics.medium_confidence = medium;
        self.statistics.low_confidence = low;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_entry_creation() {
        let entry = FileEntry::new(
            PathBuf::from("documents/report.pdf"),
            1024000,
            FilesystemType::Ext4,
            42,
        );

        assert_eq!(entry.size_bytes, 1024000);
        assert_eq!(entry.inode, 42);
        assert_eq!(entry.filesystem, FilesystemType::Ext4);
        assert!(!entry.file_id.is_empty());
    }

    #[test]
    fn test_inode_validation() {
        // Invalid: size 0 but has blocks
        let invalid = Inode {
            number: 1,
            size: 0,
            permissions: 0o644,
            owner_uid: 0,
            owner_gid: 0,
            created_time: Utc::now(),
            accessed_time: Utc::now(),
            modified_time: Utc::now(),
            blocks: vec![1, 2, 3],
            link_count: 1,
            deletion_time: None,
        };
        assert!(!invalid.is_valid());

        // Valid: size > 0 with blocks
        let valid = Inode {
            number: 1,
            size: 4096,
            permissions: 0o644,
            owner_uid: 0,
            owner_gid: 0,
            created_time: Utc::now(),
            accessed_time: Utc::now(),
            modified_time: Utc::now(),
            blocks: vec![1],
            link_count: 1,
            deletion_time: None,
        };
        assert!(valid.is_valid());
    }

    #[test]
    fn test_file_entry_json_roundtrip() {
        let entry = FileEntry::new(PathBuf::from("test.txt"), 100, FilesystemType::Ntfs, 999);

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: FileEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry, parsed);
    }

    #[test]
    fn test_recovery_index_statistics() {
        let mut index = RecoveryIndex::new("/dev/sda1".to_string(), "full".to_string());

        for i in 0..10 {
            let mut entry = FileEntry::new(
                PathBuf::from(format!("file_{}.txt", i)),
                1000 * (i + 1) as u64,
                FilesystemType::Ext4,
                i as u64,
            );
            entry.recovery_confidence = (i as f32) / 10.0;
            index.file_entries.push(entry);
        }

        index.finish();

        assert_eq!(index.statistics.total_files, 10);
        assert_eq!(index.statistics.total_bytes, 55000);
        assert!(index.statistics.high_confidence > 0);
    }

    #[test]
    fn test_deleted_file_detection() {
        let deleted = Inode {
            number: 1,
            size: 100,
            permissions: 0o644,
            owner_uid: 0,
            owner_gid: 0,
            created_time: Utc::now(),
            accessed_time: Utc::now(),
            modified_time: Utc::now(),
            blocks: vec![1],
            link_count: 0,
            deletion_time: Some(Utc::now()),
        };

        assert!(deleted.is_deleted());
    }
}
