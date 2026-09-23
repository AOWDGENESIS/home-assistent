//! UniversalRecovery Core Library
//!
//! Provides filesystem parsing, recovery algorithms, and device I/O abstraction
//! for offline data recovery on Windows, Linux, and macOS.
//!
//! # Features
//! - Multi-filesystem support (ext4, NTFS, FAT32, exFAT)
//! - Sector-by-sector recovery with 30+ file signature detection
//! - Inode chain recovery (metadata-driven reconstruction)
//! - Error correction and checksum verification
//! - Progress tracking and session persistence
//!
//! # Example
//! ```rust,no_run
//! use ur_core::DeviceReader;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Open device and start recovery scan
//!     Ok(())
//! }
//! ```

pub mod error;
pub mod filesystem;
pub mod io;
pub mod models;
pub mod recovery;

pub use error::{RecoveryError, Result};
pub use io::{DeviceReader, ImageReader};
pub use models::{DetectedFilesystem, FileEntry, FilesystemType, Inode, RecoveryIndex};
