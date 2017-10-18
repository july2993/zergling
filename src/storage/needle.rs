
use super::TTL;
use std;

pub const NeedleHeaderSize: u32 = 16;
pub const NeedlePaddingSize: u32 = 8;
pub const NeedleChecksumSize: u32 = 4;
pub const MaxPossibleVolumeSize: u32 = 4 * 1024 * 1024 * 1024 * 8;
pub const TombstoneFileSize: u32 = std::u32::MAX;
pub const PairNamePrefix: &'static str = "Zergling-";


pub struct Needle {
    Cookie: u32,
    Id: u64,
    Size: u32,

    DataSize: u32,
    Data: Vec<u8>,
    Flags: u8,
    NameSize: u8,
    Name: Vec<u8>,
    MimeSize: u8,
    Mime: Vec<u8>,
    PairsSize: u16,
    Pairs: Vec<u8>,
    LastModified: Vec<u8>,

    TTL: TTL,

    Checksum: u32,
    Padding: Vec<u8>,
}
