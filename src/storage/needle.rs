// TODO remove allow
#![allow(dead_code)]

use super::TTL;
use std;

pub const NEEDLE_HEADER_SIZE: u32 = 16;
pub const NEEDLE_PADDING_SIZE: u32 = 8;
pub const NEEDLE_CHECKSUM_SIZE: u32 = 4;
pub const MAX_POSSIBLE_VOLUME_SIZE: u64 = 4 * 1024 * 1024 * 1024 * 8;
pub const TOMBSTONE_FILE_SIZE: u32 = std::u32::MAX;
pub const PAIR_NAME_PREFIX: &'static str = "Zergling-";


pub struct Needle {
    pub cookie: u32,
    pub id: u64,
    pub size: u32,

    pub data_size: u32,
    pub data: Vec<u8>,
    pub flags: u8,
    pub name_size: u8,
    pub name: Vec<u8>,
    pub mime_size: u8,
    pub mime: Vec<u8>,
    pub pairs_size: u16,
    pub pairs: Vec<u8>,
    pub last_modified: Vec<u8>,

    pub ttl: TTL,

    pub checksum: u32,
    pub padding: Vec<u8>,
}
