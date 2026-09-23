//! Bounds-checked parsers for filesystem boot and superblock metadata.

use crate::{FilesystemType, RecoveryError, Result};

fn bytes<const N: usize>(data: &[u8], offset: usize) -> Result<&[u8; N]> {
    data.get(offset..offset + N)
        .and_then(|value| value.try_into().ok())
        .ok_or(RecoveryError::CorruptedMetadata {
            offset: offset as u64,
        })
}

fn le16(data: &[u8], offset: usize) -> Result<u16> {
    Ok(u16::from_le_bytes(*bytes::<2>(data, offset)?))
}

fn le32(data: &[u8], offset: usize) -> Result<u32> {
    Ok(u32::from_le_bytes(*bytes::<4>(data, offset)?))
}

fn le64(data: &[u8], offset: usize) -> Result<u64> {
    Ok(u64::from_le_bytes(*bytes::<8>(data, offset)?))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ext4Superblock {
    pub block_size: u32,
    pub inode_count: u32,
    pub block_count: u64,
    pub free_blocks: u64,
    pub inode_size: u16,
}

pub fn parse_ext4_superblock(data: &[u8]) -> Result<Ext4Superblock> {
    let base = 1024;
    if le16(data, base + 0x38)? != 0xEF53 {
        return Err(RecoveryError::InvalidFilesystem {
            filesystem: "ext4".to_string(),
        });
    }
    let log_block_size = le32(data, base + 0x18)?;
    let block_size =
        1024_u32
            .checked_shl(log_block_size)
            .ok_or(RecoveryError::CorruptedMetadata {
                offset: (base + 0x18) as u64,
            })?;
    Ok(Ext4Superblock {
        block_size,
        inode_count: le32(data, base)?,
        block_count: u64::from(le32(data, base + 4)?),
        free_blocks: u64::from(le32(data, base + 0x0C)?),
        inode_size: le16(data, base + 0x58)?,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsBootSector {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub mft_cluster: u64,
}

pub fn parse_ntfs_boot_sector(data: &[u8]) -> Result<NtfsBootSector> {
    if bytes::<8>(data, 3)? != b"NTFS    " {
        return Err(RecoveryError::InvalidFilesystem {
            filesystem: "NTFS".to_string(),
        });
    }
    if bytes::<2>(data, 510)? != &[0x55, 0xAA] {
        return Err(RecoveryError::CorruptedMetadata { offset: 510 });
    }
    Ok(NtfsBootSector {
        bytes_per_sector: le16(data, 11)?,
        sectors_per_cluster: bytes::<1>(data, 13)?[0],
        mft_cluster: le64(data, 48)?,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fat32BootSector {
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16,
    pub fat_count: u8,
    pub fat_size_sectors: u32,
}

pub fn parse_fat32_boot_sector(data: &[u8]) -> Result<Fat32BootSector> {
    if bytes::<2>(data, 510)? != &[0x55, 0xAA] || &bytes::<5>(data, 82)?[..] != b"FAT32" {
        return Err(RecoveryError::InvalidFilesystem {
            filesystem: "FAT32".to_string(),
        });
    }
    Ok(Fat32BootSector {
        bytes_per_sector: le16(data, 11)?,
        sectors_per_cluster: bytes::<1>(data, 13)?[0],
        reserved_sectors: le16(data, 14)?,
        fat_count: bytes::<1>(data, 16)?[0],
        fat_size_sectors: le32(data, 36)?,
    })
}

pub fn parser_filesystem_type(name: &str) -> FilesystemType {
    match name {
        "ext4" => FilesystemType::Ext4,
        "ntfs" => FilesystemType::Ntfs,
        "fat32" => FilesystemType::Fat32,
        _ => FilesystemType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ext4_superblock() {
        let mut data = vec![0; 2048];
        data[1024 + 0x38..1024 + 0x3A].copy_from_slice(&0xEF53_u16.to_le_bytes());
        data[1024 + 0x18..1024 + 0x1C].copy_from_slice(&2_u32.to_le_bytes());
        data[1024..1028].copy_from_slice(&10_u32.to_le_bytes());
        data[1024 + 0x58..1024 + 0x5A].copy_from_slice(&256_u16.to_le_bytes());
        let parsed = parse_ext4_superblock(&data).unwrap();
        assert_eq!(parsed.block_size, 4096);
        assert_eq!(parsed.inode_size, 256);
    }

    #[test]
    fn rejects_short_ntfs_sector() {
        assert!(parse_ntfs_boot_sector(&[0; 16]).is_err());
    }

    #[test]
    fn parses_ntfs_boot_sector() {
        let mut data = vec![0; 512];
        data[3..11].copy_from_slice(b"NTFS    ");
        data[11..13].copy_from_slice(&512_u16.to_le_bytes());
        data[13] = 8;
        data[48..56].copy_from_slice(&4_u64.to_le_bytes());
        data[510..512].copy_from_slice(&[0x55, 0xAA]);
        assert_eq!(parse_ntfs_boot_sector(&data).unwrap().mft_cluster, 4);
    }
}
