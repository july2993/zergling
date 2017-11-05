// TODO remove allow
#![allow(dead_code)]

use std::fs;
use std::fs::{File, metadata, Metadata};
use std::io::ErrorKind;
use std::os::unix::fs::OpenOptionsExt;
use std::time::UNIX_EPOCH;
use std::io::SeekFrom;
use std::io::Cursor;
use byteorder::WriteBytesExt;
use byteorder::{BigEndian, ReadBytesExt};
// use std::io::Read;
// use std::io::Seek;
use std::io::prelude::*;

use super::{Version, CURRENT_VERSION};
use super::ReplicaPlacement;
use super::TTL;
use super::{VolumeId, Result, Error, Needle, NeedleMapper, NeedleValueMap, NeedleMapType,
            VolumeInfo};

use super::needle::TOMBSTONE_FILE_SIZE;
use super::needle;
use time;

const SUPER_BLOCK_SIZE: usize = 8;


#[derive(Debug)]
pub struct SuperBlock {
    pub version: Version,
    pub replica_placement: ReplicaPlacement,
    pub ttl: TTL,
    pub compact_revision: u16,
}

impl Default for SuperBlock {
    fn default() -> Self {
        SuperBlock {
            version: CURRENT_VERSION,
            replica_placement: ReplicaPlacement::default(),
            ttl: TTL::default(),
            // TODO
            compact_revision: 0,
        }
    }
}


impl SuperBlock {
    pub fn parse(buf: Vec<u8>) -> Result<SuperBlock> {
        let rp = ReplicaPlacement::from_u8(buf[1])?;
        let ttl = TTL::from(buf[2..4].to_vec());
        let mut rdr = Cursor::new(buf[4..6].to_vec());
        let cr = rdr.read_u16::<BigEndian>()?;

        Ok(SuperBlock {
            version: buf[0],
            replica_placement: rp,
            ttl: ttl,
            compact_revision: cr,
        })
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.resize(SUPER_BLOCK_SIZE, 0);
        buf[0] = self.version;
        buf[1] = self.replica_placement.byte();

        let mut idx = 2;
        for u in self.ttl.bytes() {
            buf[idx] = u;
            idx += 1;
        }

        {
            let mut r = &mut buf[4..6];
            r.write_u16::<BigEndian>(self.compact_revision).unwrap();
        }


        buf
    }
}



#[derive(Default)]
pub struct Volume {
    pub id: VolumeId,
    pub dir: String,
    pub collection: String,
    pub data_file: Option<File>,
    pub nm: NeedleMapper,
    pub needle_map_kind: NeedleMapType,
    pub replica_placement: ReplicaPlacement,

    pub read_only: bool,

    pub super_block: SuperBlock,

    pub last_modified_time: u64,

    pub last_compact_index_offset: u64,
    pub last_compact_revision: u16,
}


impl Volume {
    pub fn new(
        dir: &str,
        collection: &str,
        id: VolumeId,
        needle_map_kind: NeedleMapType,
        replica_placement: ReplicaPlacement,
        ttl: TTL,
        pre_allocate: i64,
    ) -> Result<Volume> {

        let sb = SuperBlock {
            replica_placement: replica_placement,
            ttl: ttl,
            ..Default::default()
        };



        // Err(box_err!("todo"))
        let mut v = Volume {
            dir: dir.to_string(),
            collection: collection.to_string(),
            id: id,
            super_block: sb,
            data_file: None,
            needle_map_kind: needle_map_kind,
            ..Default::default()
        };

        v.load(true, true)?;
        Ok(v)
    }

    fn load(&mut self, create_if_missing: bool, load_index: bool) -> Result<()> {
        if self.data_file.is_some() {
            return Err(box_err!("has load!"));
        }


        let mut name = self.file_name();
        name.push_str(".dat");


        let mut meta: Metadata;

        match metadata(&self.file_name()) {
            Ok(m) => meta = m,
            Err(err) => {
                if err.kind() == ErrorKind::NotFound && create_if_missing {
                    // TODO suppose pre_allocate
                    let file = fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .mode(0o644)
                        .open(&name)?;
                    meta = metadata(&name)?;
                    self.write_super_block()?;
                } else {
                    return Err(Error::from(err));
                }
            }
        };

        if !meta.permissions().readonly() {

            let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .mode(0o644)
                .open(&name)?;

            self.last_modified_time = meta.modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.data_file = Some(file);
        } else {
            let file = fs::OpenOptions::new().read(true).open(&name);

            self.read_only = true;
        }

        self.read_super_block()?;

        if load_index {}

        Ok(())
    }

    fn write_super_block(&mut self) -> Result<()> {
        let bytes = self.super_block.bytes();
        let file = self.file_mut();
        let meta = file.metadata()?;

        if meta.len() != 0 {
            return Ok(());
        }


        file.write_all(&bytes)?;



        Ok(())
    }

    fn file_mut(&mut self) -> &mut File {
        self.data_file.as_mut().unwrap()
    }

    fn read_super_block(&mut self) -> Result<()> {
        let mut buf: Vec<u8> = vec![];

        {
            let file = self.file_mut();
            file.seek(SeekFrom::Start(0))?;

            buf.resize(SUPER_BLOCK_SIZE, 0);
            file.read_exact(&mut buf)?;
        }
        self.super_block = SuperBlock::parse(buf)?;

        Ok(())
    }


    pub fn write_needle(&mut self, n: &Needle) -> Result<u32> {
        panic!("TODO");
    }

    pub fn delete_needle(&mut self, n: &Needle) -> Result<u32> {
        panic!("TODO");
    }

    pub fn read_needle(&mut self, n: &mut Needle) -> Result<u32> {
        let nv = self.nm.get(n.id).ok_or::<Error>(box_err!("Not Found"))?;

        if nv.size == TOMBSTONE_FILE_SIZE {
            return Err(box_err!("Already Deleted"));
        }

        let version = self.version();
        n.read_date(
            &mut self.data_file.as_mut().unwrap(),
            nv.offset,
            nv.size,
            version,
        )?;

        let nread = n.data.len();

        if n.has_ttl() && n.has_last_modified_date() {
            let minitus = n.ttl.minutes();
            if minitus > 0 {
                if time::get_time().sec >= (n.last_modified + minitus as u64 * 60) as i64 {
                    return Err(box_err!("Not Found"));
                }
            }
        }

        Ok(nread as u32)
    }

    fn version(&self) -> Version {
        self.super_block.version
    }

    pub fn file_name(&self) -> String {
        let mut rt = self.dir.clone();
        if self.collection == "" {
            rt.push_str(&self.id.to_string())
        } else {
            rt.push_str(&self.collection);
            rt.push_str("_");
            rt.push_str(&self.id.to_string())
        }

        rt

    }

    pub fn get_volume_info(&self) -> VolumeInfo {
        VolumeInfo {
            id: self.id,
            size: self.content_size(),
            replica_placement: self.super_block.replica_placement,
            ttl: self.super_block.ttl,
            collection: self.collection.clone(),
            version: self.version(),
            file_count: self.nm.file_count() as i64,
            delete_count: self.nm.delete_count() as i64,
            delete_byte_count: self.nm.delete_byte_count(),
            read_only: self.read_only,
        }
    }

    pub fn content_size(&self) -> u64 {
        0
    }

    pub fn size() -> i64 {
        panic!("TODO");
        0
    }
}
