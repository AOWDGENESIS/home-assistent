//! Device I/O abstraction layer

use async_trait::async_trait;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::{RecoveryError, Result};

/// Abstract device reader trait for multi-platform support
#[async_trait]
pub trait DeviceReader: Send + Sync {
    /// Read sectors from device
    async fn read_sector(&self, sector: u64, count: u32) -> Result<Vec<u8>>;

    /// Get total device size in bytes
    async fn get_size(&self) -> Result<u64>;

    /// Get block size (usually 512 or 4096)
    async fn get_block_size(&self) -> Result<u32>;
}

/// Read-only sector access to a regular disk-image file.
///
/// This is the safe test and preview path. It never opens a physical device.
pub struct ImageReader {
    file: std::sync::Mutex<File>,
    path: PathBuf,
    size: u64,
    block_size: u32,
}

impl ImageReader {
    /// Open an existing image with a 512-byte logical sector size.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path).map_err(RecoveryError::from)?;
        let size = file.metadata().map_err(RecoveryError::from)?.len();
        Ok(Self {
            file: std::sync::Mutex::new(file),
            path,
            size,
            block_size: 512,
        })
    }

    /// Return the source image path.
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[async_trait]
impl DeviceReader for ImageReader {
    async fn read_sector(&self, sector: u64, count: u32) -> Result<Vec<u8>> {
        let bytes = u64::from(self.block_size)
            .checked_mul(u64::from(count))
            .ok_or_else(|| RecoveryError::FileTooLarge { size: u64::MAX })?;
        let offset = sector
            .checked_mul(u64::from(self.block_size))
            .ok_or_else(|| RecoveryError::SectorUnreadable { sector })?;
        let end = offset
            .checked_add(bytes)
            .ok_or_else(|| RecoveryError::SectorUnreadable { sector })?;
        if end > self.size {
            return Err(RecoveryError::SectorUnreadable { sector });
        }
        let mut buffer = vec![
            0;
            usize::try_from(bytes)
                .map_err(|_| RecoveryError::FileTooLarge { size: bytes })?
        ];
        let mut file = self.file.lock().map_err(|_| {
            RecoveryError::IoError("Image-Datei konnte nicht gesperrt werden".to_string())
        })?;
        file.seek(SeekFrom::Start(offset))
            .map_err(RecoveryError::from)?;
        file.read_exact(&mut buffer).map_err(RecoveryError::from)?;
        Ok(buffer)
    }

    async fn get_size(&self) -> Result<u64> {
        Ok(self.size)
    }

    async fn get_block_size(&self) -> Result<u32> {
        Ok(self.block_size)
    }
}

pub mod cache;
pub use cache::SectorCache;
