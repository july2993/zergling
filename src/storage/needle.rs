#![allow(dead_code)]

use super::{Result, Version, TTL};
use super::version::VERSION2;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use crc::crc32;
use std::io::SeekFrom;
use std::fmt::{self, Debug, Display, Formatter};
use byteorder::WriteBytesExt;
use byteorder::ByteOrder;

pub const NEEDLE_HEADER_SIZE: u32 = 16;
pub const NEEDLE_PADDING_SIZE: u32 = 8;
pub const NEEDLE_CHECKSUM_SIZE: u32 = 4;
pub const MAX_POSSIBLE_VOLUME_SIZE: u64 = 4 * 1024 * 1024 * 1024 * 8;
pub const PAIR_NAME_PREFIX: &'static str = "Zergling-";
pub const FLAG_GZIP: u8 = 0x01;
pub const FLAG_HAS_NAME: u8 = 0x02;
pub const FLAG_HAS_MIME: u8 = 0x04;
pub const FLAG_HAS_LAST_MODIFIED_DATE: u8 = 0x08;
pub const FLAG_HAS_TTL: u8 = 0x10;
pub const FLAG_HAS_PAIRS: u8 = 0x20;
pub const FLAG_IS_DELETE: u8 = 0x40;
pub const FLAG_IS_CHUNK_MANIFEST: u8 = 0x80;

pub const LAST_MODIFIED_BYTES_LENGTH: usize = 8;
pub const TTL_BYTES_LENGTH: usize = 2;

pub const NEEDLE_FLAG_OFFSET: usize = 20;
pub const NEEDLE_ID_OFFSET: usize = 4;
pub const NEEDLE_SIZE_OFFSET: usize = 12;

#[derive(Debug, Default)]
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

impl Display for Needle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}, cookie: {} size: {}, data_len: {}, has_pairs: {} flag: {}",
            self.id,
            self.cookie,
            self.size,
            self.data.len(),
            self.has_pairs(),
            self.flags,
        )
    }
}

pub fn get_actual_size(size: u32) -> u64 {
    let padding: u64;
    let left = (NEEDLE_HEADER_SIZE + size + NEEDLE_CHECKSUM_SIZE) % NEEDLE_PADDING_SIZE as u32;
    if left > 0 {
        padding = NEEDLE_PADDING_SIZE as u64 - left as u64;
    } else {
        padding = 0;
    }

    NEEDLE_HEADER_SIZE as u64 + size as u64 + NEEDLE_CHECKSUM_SIZE as u64 + padding
}

pub fn true_offset(offset: u32) -> u64 {
    offset as u64 * NEEDLE_PADDING_SIZE as u64
}

fn read_needle_blob(file: &mut File, offset: u32, size: u32) -> Result<Vec<u8>> {
    let mut buffer: Vec<u8> = vec![];
    buffer.resize(get_actual_size(size) as usize, 0);
    file.seek(SeekFrom::Start(true_offset(offset)))?;

    debug!("read {} at {}", get_actual_size(size), true_offset(offset));
    file.read_exact(&mut buffer)?;
    debug!("read success");

    Ok(buffer)
}

impl Needle {
    pub fn parse_needle_header(&mut self, bytes: &Vec<u8>) {
        let mut rdr = Cursor::new(bytes);
        self.cookie = rdr.read_u32::<BigEndian>().unwrap();
        self.id = rdr.read_u64::<BigEndian>().unwrap();
        self.size = rdr.read_u32::<BigEndian>().unwrap();
    }

    pub fn parse_path(&mut self, fid: &str) -> Result<()> {
        if fid.len() <= 8 {
            return Err(box_err!("invalid fid: {}", fid));
        }

        debug!("parse_pat fid: {}", fid);

        let id: &str;
        let delta: &str;
        if let Some(idx) = fid.find("_") {
            id = &fid[0..idx];
            delta = &fid[idx + 1..];
        } else {
            id = &fid[0..fid.len()];
            delta = &fid[0..0];
        }

        debug!("parse_pat id: {} delta: {}", id, delta);

        let ret = parse_key_hash(id)?;
        self.id = ret.0;
        self.cookie = ret.1;
        if delta.len() > 0 {
            let idelta: u64 = delta.parse()?;
            self.id += idelta;
        }

        debug!("parse result id: {}, cookie: {}", self.id, self.cookie);

        Ok(())
    }


    // TODO: avoid data copy
    fn read_needle_data(&mut self, bytes: &[u8]) {
        let mut idx = 0;
        let len = bytes.len();

        if idx < len {
            self.data_size = BigEndian::read_u32(&bytes[idx..idx + 4]);
            idx += 4;


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
            // idx += self.pairs_size as usize;
        }
    }

    pub fn append<W: std::io::Write>(&mut self, w: &mut W, version: Version) -> Result<(u32, u64)> {
        if version != super::CURRENT_VERSION {
            return Err(box_err!("no supported version"));
        }

        let mut bytes: Vec<u8> = vec![];
        bytes.write_u32::<BigEndian>(self.cookie).unwrap();
        bytes.write_u64::<BigEndian>(self.id).unwrap();

        self.data_size = self.data.len() as u32;
        self.name_size = self.name.len() as u8;
        self.mime_size = self.mime.len() as u8;

        if self.data_size > 0 {
            self.size = 4 + self.data_size + 1; // one for flag;
            if self.has_name() {
                self.size += 1 + self.name_size as u32;
            }
            if self.has_mime() {
                self.size += 1 + self.mime_size as u32;
            }
            if self.has_last_modified_date() {
                self.size += LAST_MODIFIED_BYTES_LENGTH as u32;
            }
            if self.has_ttl() {
                self.size += TTL_BYTES_LENGTH as u32;
            }
            if self.has_pairs() {
                self.size += self.pairs_size as u32;
            }
        } else {
            self.size = 0
        }

        bytes.write_u32::<BigEndian>(self.size).unwrap();
        w.write_all(&bytes)?;
        bytes.clear();

        if self.data_size > 0 {
            bytes.write_u32::<BigEndian>(self.data_size).unwrap();
            w.write_all(&bytes)?;
            bytes.clear();

            w.write_all(&self.data)?;

            w.write_all(&vec![self.flags])?;

            if self.has_name() {
                w.write_all(&vec![self.name_size])?;
                w.write_all(&self.name)?;
            }

            if self.has_mime() {
                w.write_all(&vec![self.mime_size])?;
                w.write_all(&self.mime)?;
            }
            if self.has_last_modified_date() {
                bytes.write_u64::<BigEndian>(self.last_modified).unwrap();
                w.write_all(&bytes)?;
                bytes.clear();
            }

            if self.has_ttl() {
                w.write_all(&self.ttl.bytes())?;
            }

            // not supporst
            if self.has_pairs() {
                panic!("not suppose");
            }
        }

        let mut padding = 0;
        if (NEEDLE_HEADER_SIZE + self.size + NEEDLE_CHECKSUM_SIZE) % NEEDLE_PADDING_SIZE != 0 {
            padding = NEEDLE_PADDING_SIZE
                - (NEEDLE_HEADER_SIZE + self.size + NEEDLE_CHECKSUM_SIZE) % NEEDLE_PADDING_SIZE;
        }

        bytes.write_u32::<BigEndian>(self.checksum).unwrap();
        w.write_all(&bytes)?;
        bytes.clear();

        w.write_all(&vec![0; padding as usize])?;

        Ok((self.data_size, get_actual_size(self.size)))
    }

    pub fn read_date(
        &mut self,
        file: &mut File,
        offset: u32,
        size: u32,
        version: Version,
    ) -> Result<()> {
        let bytes = read_needle_blob(file, offset, size)?;
        self.parse_needle_header(&bytes);
        debug!(
            "parse header success cookie: {}, id: {}, size: {}",
            self.cookie,
            self.id,
            self.size
        );

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
            &bytes[(NEEDLE_HEADER_SIZE + size) as usize
                       ..(NEEDLE_HEADER_SIZE + size + NEEDLE_CHECKSUM_SIZE) as usize],
        ).read_u32::<BigEndian>()
            .unwrap();
        let cal_checksum = crc32::checksum_castagnoli(&self.data);

        if self.checksum != cal_checksum {
            return Err(box_err!(
                "CRC error, read: {}, calucate: {} may be data on disk corrupted",
                self.checksum,
                cal_checksum
            ));
        }

        Ok(())
    }

    pub fn has_ttl(&self) -> bool {
        self.flags & FLAG_HAS_TTL > 0
    }
    pub fn set_has_ttl(&mut self) {
        self.flags |= FLAG_HAS_TTL
    }
    pub fn has_name(&self) -> bool {
        self.flags & FLAG_HAS_NAME > 0
    }
    pub fn set_name(&mut self) {
        self.flags |= FLAG_HAS_NAME
    }
    pub fn has_mime(&self) -> bool {
        self.flags & FLAG_HAS_MIME > 0
    }
    pub fn set_has_mime(&mut self) {
        self.flags |= FLAG_HAS_MIME
    }
    pub fn is_gzipped(&self) -> bool {
        self.flags & FLAG_GZIP > 0
    }
    pub fn set_gzipped(&mut self) {
        self.flags |= FLAG_GZIP
    }
    pub fn has_pairs(&self) -> bool {
        self.flags & FLAG_HAS_PAIRS > 0
    }

    pub fn has_last_modified_date(&self) -> bool {
        self.flags & FLAG_HAS_LAST_MODIFIED_DATE > 0
    }

    pub fn set_has_last_modified_date(&mut self) {
        self.flags |= FLAG_HAS_LAST_MODIFIED_DATE
    }

    pub fn set_is_chunk_manifest(&mut self) {
        self.flags |= FLAG_IS_CHUNK_MANIFEST
    }

    pub fn is_chunk_manifest(&self) -> bool {
        self.flags & FLAG_IS_CHUNK_MANIFEST > 0
    }

    pub fn is_delete(&self) -> bool {
        self.flags & FLAG_IS_DELETE > 0
    }

    pub fn set_is_delete(&mut self) {
        self.flags |= FLAG_IS_DELETE
    }

    pub fn etag(&self) -> String {
        let mut buf: Vec<u8> = vec![0; 4];
        {
            let mut r = &mut buf[0..4];
            r.write_u32::<BigEndian>(self.checksum).unwrap();
        }
        format!("{}{}{}{}", buf[0], buf[1], buf[2], buf[3])
    }
}


fn parse_key_hash(hash: &str) -> Result<(u64, u32)> {
    if hash.len() <= 8 || hash.len() > 24 {
        return Err(box_err!("key_hash too short or too long: {}", hash));
    }

    let key_end = hash.len() - 8;

    let key: u64 = u64::from_str_radix(&hash[0..key_end], 16)?;
    let cookie: u32 = u32::from_str_radix(&hash[key_end..], 16)?;

    Ok((key, cookie))
}
