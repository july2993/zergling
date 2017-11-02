// TODO remove allow
#![allow(dead_code)]

use super::{TTL, Result, Version};
use super::version::VERSION2;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crc::{crc32, Hasher32};
use std::io::SeekFrom;




pub const NEEDLE_HEADER_SIZE: u32 = 16;
pub const NEEDLE_PADDING_SIZE: u32 = 8;
pub const NEEDLE_CHECKSUM_SIZE: u32 = 4;
pub const MAX_POSSIBLE_VOLUME_SIZE: u64 = 4 * 1024 * 1024 * 1024 * 8;
pub const TOMBSTONE_FILE_SIZE: u32 = std::u32::MAX;
pub const PAIR_NAME_PREFIX: &'static str = "Zergling-";


pub const FLAG_GZIP: u8 = 0x01;
pub const FLAG_HAS_NAME: u8 = 0x02;
pub const FLAG_HAS_MIME: u8 = 0x04;
pub const FLAG_HAS_LAST_MODIFIED_DATE: u8 = 0x08;
pub const FLAG_HAS_TTL: u8 = 0x10;
pub const FLAG_HAS_PAIRS: u8 = 0x20;
pub const FLAG_IS_CHUNK_MANIFEST: u8 = 0x80;

pub const LAST_MODIFIED_BYTES_LENGTH: usize = 5;
pub const TTL_BYTES_LENGTH: usize = 2;

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
    pub last_modified: u64,

    pub ttl: TTL,

    pub checksum: u32,
    pub padding: Vec<u8>,
}

fn get_actual_size(size: u32) -> u64 {
    let padding: u64;
    let left = (NEEDLE_HEADER_SIZE + size + NEEDLE_CHECKSUM_SIZE) % NEEDLE_PADDING_SIZE;
    if left > 0 {
        padding = NEEDLE_PADDING_SIZE as u64 - left as u64;
    } else {
        padding = 0;
    }

    NEEDLE_HEADER_SIZE as u64 + size as u64 + NEEDLE_CHECKSUM_SIZE as u64 + padding
}

fn read_needle_blob(file: &mut File, offset: u32, size: u32) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![];
    buffer.resize(get_actual_size(size) as usize, 0);
    file.seek(SeekFrom::Start(offset as u64))?;
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

impl Needle {
    pub fn parse_needle_header(&mut self, bytes: &Vec<u8>) {
        let mut rdr = Cursor::new(bytes);
        self.cookie = rdr.read_u32::<BigEndian>().unwrap();
        self.id = rdr.read_u64::<BigEndian>().unwrap();
        self.size = rdr.read_u32::<BigEndian>().unwrap();
    }


    // TODO: avoid data copy
    fn read_needle_data(&mut self, bytes: &[u8]) {
        let mut idx = 0;
        let len = bytes.len();

        if idx < len {
            self.data = bytes[idx..idx + self.data_size as usize].to_vec();
            idx += self.data_size as usize;
            self.flags = bytes[idx];
            idx += 1;
        }

        if idx < len && self.has_name() {
            self.name_size = bytes[idx] as u8;
            idx += 1;
            self.name = bytes[idx..idx + self.name_size as usize].to_vec();
            idx += self.name_size as usize;
        }

        if idx < len && self.has_mime() {
            self.mime_size = bytes[idx] as u8;
            idx += 1;
            self.mime = bytes[idx..idx + self.mime_size as usize].to_vec();
            idx += self.mime_size as usize;
        }


        if idx < len && self.has_last_modified_date() {
            // TODO not enough 8 bytes may panic?
            self.last_modified = Cursor::new(bytes[idx..idx + LAST_MODIFIED_BYTES_LENGTH].to_vec())
                .read_u64::<BigEndian>()
                .unwrap();
            idx += LAST_MODIFIED_BYTES_LENGTH;
        }

        if idx < len && self.has_ttl() {
            self.ttl = TTL::from(bytes[idx..idx + TTL_BYTES_LENGTH as usize].to_vec());
            idx += TTL_BYTES_LENGTH as usize;
        }

        if idx < len && self.has_pairs() {
            self.pairs_size = Cursor::new(bytes[idx..idx + 1].to_vec())
                .read_u16::<BigEndian>()
                .unwrap();
            idx += 2;
            self.pairs = bytes[idx..idx + self.pairs_size as usize].to_vec();
            idx += self.pairs_size as usize;
        }

    }

    //
    pub fn read_date(
        &mut self,
        file: &mut File,
        offset: u32,
        size: u32,
        version: Version,
    ) -> Result<()> {
        let bytes = read_needle_blob(file, offset, size)?;
        self.parse_needle_header(&bytes);
        if self.size != size {
            return Err(box_err!(
                "file entry not found. needle {} memory {}",
                self.size,
                size
            ));
        }

        match version {
            VERSION2 => {
                let end = (NEEDLE_HEADER_SIZE + self.size) as usize;
                self.read_needle_data(&bytes[NEEDLE_HEADER_SIZE as usize..end]);
            }
            _ => (),
        };

        self.checksum = Cursor::new(
            &bytes[(NEEDLE_HEADER_SIZE + size) as usize..
                       (NEEDLE_HEADER_SIZE + size + NEEDLE_CHECKSUM_SIZE) as usize],
        ).read_u32::<BigEndian>()
            .unwrap();
        let cal_checksum = crc32::checksum_castagnoli(&self.data);

        if self.checksum != cal_checksum {
            return Err(box_err!("CRC error, may be data on disk corrupted"));
        }

        Ok(())
    }

    pub fn has_ttl(&self) -> bool {
        self.flags | FLAG_HAS_TTL > 0
    }
    pub fn has_name(&self) -> bool {
        self.flags | FLAG_HAS_NAME > 0
    }
    pub fn has_mime(&self) -> bool {
        self.flags | FLAG_HAS_MIME > 0
    }
    pub fn is_gzipped(&self) -> bool {
        self.flags | FLAG_GZIP > 0
    }
    pub fn has_pairs(&self) -> bool {
        self.flags | FLAG_HAS_PAIRS > 0
    }

    pub fn has_last_modified_date(&self) -> bool {
        self.flags | FLAG_HAS_LAST_MODIFIED_DATE > 0
    }
}
