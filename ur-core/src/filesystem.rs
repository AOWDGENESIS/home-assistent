//! Filesystem abstraction and lightweight signature detection.

use crate::{FilesystemType, Result};
use async_trait::async_trait;
use std::path::Path;

use crate::io::DeviceReader;
use crate::models::{FileEntry, Inode};

pub mod parsers;
pub use parsers::{
    parse_ext4_superblock, parse_fat32_boot_sector, parse_ntfs_boot_sector, Ext4Superblock,
    Fat32BootSector, NtfsBootSector,
};

/// Filesystem reader implemented by each supported filesystem parser.
#[async_trait]
pub trait FilesystemReader: Send + Sync {
    /// Return the detected filesystem type.
    fn filesystem_type(&self) -> FilesystemType;
    /// List files below a directory.
    async fn list_files(&self, directory: &Path) -> Result<Vec<FileEntry>>;
    /// Read one filesystem-specific inode.
    async fn read_inode(&self, inode: u64) -> Result<Inode>;
}

/// Detect a filesystem from common magic values at an offset.
pub async fn detect_filesystem(reader: &dyn DeviceReader, offset: u64) -> Result<FilesystemType> {
    let block_size = reader.get_block_size().await?;
    let data = reader
        .read_sector(offset / u64::from(block_size), 3)
        .await?;
    if data.len() >= 0x43a && data[0x438..0x43a] == [0x53, 0xef] {
        return Ok(FilesystemType::Ext4);
    }
    if data.len() >= 11 && &data[3..11] == b"NTFS    " {
        return Ok(FilesystemType::Ntfs);
    }
    if data.len() >= 90 && &data[82..90] == b"FAT32   " {
        return Ok(FilesystemType::Fat32);
    }
    Ok(FilesystemType::Unknown)
}
