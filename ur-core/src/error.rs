//! Comprehensive error taxonomy for UniversalRecovery
//!
//! All errors are serializable and include German user-facing messages.

use serde::{Deserialize, Serialize};
use std::io;
use thiserror::Error;

/// Main result type for UniversalRecovery operations
pub type Result<T> = std::result::Result<T, RecoveryError>;

/// Comprehensive recovery error taxonomy with German UI strings
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "error_code", content = "details")]
pub enum RecoveryError {
    /// Device not found or not accessible
    #[error("Device not found: {0}")]
    #[serde(rename = "ER_DEVICE_NOT_FOUND")]
    DeviceNotFound(String),

    /// Permission denied (requires root/admin)
    #[error("Permission denied")]
    #[serde(rename = "ER_PERMISSION_DENIED")]
    PermissionDenied,

    /// Read timeout at specified sector
    #[error("Read timeout at sector {sector}")]
    #[serde(rename = "ER_READ_TIMEOUT")]
    ReadTimeout { sector: u64 },

    /// I/O error with context
    #[error("I/O error: {0}")]
    #[serde(rename = "ER_IO_ERROR")]
    IoError(String),

    /// Checksum verification failed
    #[error("Checksum failed at inode {inode}")]
    #[serde(rename = "ER_CHECKSUM_FAILED")]
    ChecksumFailed { inode: u64 },

    /// Filesystem magic number incorrect
    #[error("Invalid filesystem: {filesystem}")]
    #[serde(rename = "ER_INVALID_FILESYSTEM")]
    InvalidFilesystem { filesystem: String },

    /// Corrupted superblock or metadata
    #[error("Corrupted metadata at offset {offset}")]
    #[serde(rename = "ER_CORRUPTED_METADATA")]
    CorruptedMetadata { offset: u64 },

    /// Recovery confidence below threshold
    #[error("Low confidence recovery: {confidence:.2}")]
    #[serde(rename = "ER_LOW_CONFIDENCE")]
    LowConfidence { confidence: f32 },

    /// File too large for current buffer
    #[error("File too large: {size} bytes")]
    #[serde(rename = "ER_FILE_TOO_LARGE")]
    FileTooLarge { size: u64 },

    /// Sector unreadable, skipped
    #[error("Unreadable sector: {sector}")]
    #[serde(rename = "ER_SECTOR_UNREADABLE")]
    SectorUnreadable { sector: u64 },

    /// Inode chain incomplete or broken
    #[error("Broken inode chain at block {block}")]
    #[serde(rename = "ER_BROKEN_INODE_CHAIN")]
    BrokenInodeChain { block: u64 },

    /// Allocation error (out of memory)
    #[error("Allocation error: {0}")]
    #[serde(rename = "ER_ALLOCATION_ERROR")]
    AllocationError(String),

    /// Session not found or corrupted
    #[error("Invalid session: {session_id}")]
    #[serde(rename = "ER_INVALID_SESSION")]
    InvalidSession { session_id: String },

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    #[serde(rename = "ER_SERIALIZATION_ERROR")]
    SerializationError(String),

    /// Generic unrecoverable error
    #[error("Unrecoverable error: {0}")]
    #[serde(rename = "ER_UNRECOVERABLE")]
    Unrecoverable(String),
}

impl RecoveryError {
    /// Get German user-facing error message
    pub fn user_message_de(&self) -> String {
        match self {
            RecoveryError::DeviceNotFound(dev) => {
                format!("Laufwerk '{}' nicht gefunden.", dev)
            }
            RecoveryError::PermissionDenied => {
                "Zugriff verweigert. Administrator-Rechte erforderlich.".to_string()
            }
            RecoveryError::ReadTimeout { sector } => {
                format!(
                    "Lesefehler bei Sektor {}. Wiederholung wird versucht.",
                    sector
                )
            }
            RecoveryError::IoError(msg) => {
                format!("Eingabe-/Ausgabefehler: {}", msg)
            }
            RecoveryError::ChecksumFailed { inode } => {
                format!("Checksumme-Fehler bei Inode {}.", inode)
            }
            RecoveryError::InvalidFilesystem { filesystem } => {
                format!(
                    "Dateisystem '{}' ungültig oder nicht unterstützt.",
                    filesystem
                )
            }
            RecoveryError::CorruptedMetadata { offset } => {
                format!(
                    "Beschädigte Metadaten bei Offset {}. Versuche Wiederherstellung.",
                    offset
                )
            }
            RecoveryError::LowConfidence { confidence } => {
                format!(
                    "Wiederherstellungsconfidence niedrig ({:.0}%). Möglicherweise fehlerhaft.",
                    confidence * 100.0
                )
            }
            RecoveryError::FileTooLarge { size } => {
                format!("Datei zu groß: {} Bytes.", size)
            }
            RecoveryError::SectorUnreadable { sector } => {
                format!("Sektor {} nicht lesbar. Überspringe.", sector)
            }
            RecoveryError::BrokenInodeChain { block } => {
                format!("Inode-Kette unterbrochen bei Block {}.", block)
            }
            RecoveryError::AllocationError(reason) => {
                format!("Speicherzuweisungsfehler: {}", reason)
            }
            RecoveryError::InvalidSession { session_id } => {
                format!("Session '{}' ungültig oder beschädigt.", session_id)
            }
            RecoveryError::SerializationError(reason) => {
                format!("Serialisierungsfehler: {}", reason)
            }
            RecoveryError::Unrecoverable(msg) => {
                format!("Nicht wiederherstellbarer Fehler: {}", msg)
            }
        }
    }

    /// Get error code for CLI/API responses
    pub fn error_code(&self) -> &'static str {
        match self {
            RecoveryError::DeviceNotFound(_) => "ER_DEVICE_NOT_FOUND",
            RecoveryError::PermissionDenied => "ER_PERMISSION_DENIED",
            RecoveryError::ReadTimeout { .. } => "ER_READ_TIMEOUT",
            RecoveryError::IoError(_) => "ER_IO_ERROR",
            RecoveryError::ChecksumFailed { .. } => "ER_CHECKSUM_FAILED",
            RecoveryError::InvalidFilesystem { .. } => "ER_INVALID_FILESYSTEM",
            RecoveryError::CorruptedMetadata { .. } => "ER_CORRUPTED_METADATA",
            RecoveryError::LowConfidence { .. } => "ER_LOW_CONFIDENCE",
            RecoveryError::FileTooLarge { .. } => "ER_FILE_TOO_LARGE",
            RecoveryError::SectorUnreadable { .. } => "ER_SECTOR_UNREADABLE",
            RecoveryError::BrokenInodeChain { .. } => "ER_BROKEN_INODE_CHAIN",
            RecoveryError::AllocationError(_) => "ER_ALLOCATION_ERROR",
            RecoveryError::InvalidSession { .. } => "ER_INVALID_SESSION",
            RecoveryError::SerializationError(_) => "ER_SERIALIZATION_ERROR",
            RecoveryError::Unrecoverable(_) => "ER_UNRECOVERABLE",
        }
    }
}

impl From<io::Error> for RecoveryError {
    fn from(err: io::Error) -> Self {
        RecoveryError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for RecoveryError {
    fn from(err: serde_json::Error) -> Self {
        RecoveryError::SerializationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_serialization() {
        let err = RecoveryError::DeviceNotFound("/dev/sda".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let parsed: RecoveryError = serde_json::from_str(&json).unwrap();
        assert_eq!(err.error_code(), parsed.error_code());
    }

    #[test]
    fn test_german_messages() {
        let err = RecoveryError::PermissionDenied;
        let msg = err.user_message_de();
        assert!(msg.contains("Admin"));

        let err = RecoveryError::ReadTimeout { sector: 12345 };
        let msg = err.user_message_de();
        assert!(msg.contains("12345"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let recovery_err = RecoveryError::from(io_err);
        assert!(matches!(recovery_err, RecoveryError::IoError(_)));
    }
}
