// TODO remove allow
#![allow(dead_code)]

use std::{thread, fs};
use std::path::Path;
use std::sync::mpsc;
use std::fs::{File, metadata, Metadata};
use std::time::{Duration, SystemTime};
use std::io::ErrorKind;
use std::os::unix::fs::OpenOptionsExt;
use std::time::UNIX_EPOCH;
use storage::needle::NEEDLE_PADDING_SIZE;
use std::io::SeekFrom;
use std::io::Cursor;
use storage::needle_value::NeedleValue;
// use std::os::ext::fs::MetadataExt;
// use std::os::unix::fs::MetadataExt;
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

use super::needle;
use time;

pub const SUPER_BLOCK_SIZE: usize = 8;


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
        let mut buf: Vec<u8> = vec![0; SUPER_BLOCK_SIZE];
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

// #[derive(Default)]
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
    index_sender: mpsc::Sender<(u64, u32, u32)>,
}

impl Volume {
    pub fn new(
        dir: &str,
        collection: &str,
        id: VolumeId,
        needle_map_kind: NeedleMapType,
        replica_placement: ReplicaPlacement,
        ttl: TTL,
        _pre_allocate: i64,
    ) -> Result<Volume> {
        debug!("new volume dir: {}, id: {}", dir, id);
        let sb = SuperBlock {
            replica_placement: replica_placement,
            ttl: ttl,
            ..Default::default()
        };

        let (send, recv) = mpsc::channel();


        // Err(box_err!("todo"))
        let mut v = Volume {
            id: id,
            dir: dir.to_string(),
            collection: collection.to_string(),
            super_block: sb,
            data_file: None,
            needle_map_kind: needle_map_kind,
            index_sender: send,
            nm: NeedleMapper::default(),
            read_only: false,
            last_compact_index_offset: 0,
            last_compact_revision: 0,
            last_modified_time: 0,
            replica_placement: ReplicaPlacement::default(),
            
            // ..Default::default()
        };

        v.load(true, true)?;
        v.spawn_index_file_writer(recv)?;
        debug!("new volume dir: {}, id: {} load success", dir, id);
        Ok(v)
    }

    fn spawn_index_file_writer(&self, rx: mpsc::Receiver<(u64, u32, u32)>) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(self.index_file_name())?;

        let vid = self.id;

        thread::spawn(move || {
            let mut bytes: Vec<u8> = vec![];
            for v in rx.iter() {
                bytes.write_u64::<BigEndian>(v.0).unwrap();
                bytes.write_u32::<BigEndian>(v.1).unwrap();
                bytes.write_u32::<BigEndian>(v.2).unwrap();

                if bytes.len() >= 1024 {
                    if let Ok(_) = file.write_all(&bytes) {
                        bytes.clear();
                    } else {
                        warn!("write all fail, volume {} index file writer exit", vid);
                        return;
                    }
                }

            }
            warn!("volume {} index file writer exit", vid);

        });

        Ok(())
    }


    fn load(&mut self, create_if_missing: bool, load_index: bool) -> Result<()> {
        if self.data_file.is_some() {
            return Err(box_err!("has load!"));
        }

        let meta: Metadata;
        let name = self.data_file_name();
        debug!("load volume: {}", name);

        match metadata(&name) {
            Ok(m) => meta = m,
            Err(err) => {
                debug!("get metadata err: {}", err);
                if err.kind() == ErrorKind::NotFound && create_if_missing {
                    // TODO suppose pre_allocate
                    fs::OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .mode(0o644)
                        .open(&name)?;
                    meta = metadata(&name)?;
                } else {
                    return Err(Error::from(err));
                }
            }
        };

        // debug!("meta: {:?}", meta);

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
            let file = fs::OpenOptions::new().read(true).open(&name)?;
            self.data_file = Some(file);

            self.read_only = true;
        }

        self.write_super_block()?;

        self.read_super_block()?;
        debug!("read_super_block success");

        if load_index {
            let mut index_file = fs::OpenOptions::new().read(true).open(
                self.index_file_name(),
            )?;

            let mut data_file = fs::OpenOptions::new().read(true).open(
                self.data_file_name(),
            )?;

            self.nm.load_idx_file(&mut index_file, &mut data_file)?;
        }

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
        debug!("write super block success");
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

    fn async_wirte_index_file(&mut self, key: u64, offset: u32, size: u32) -> Result<()> {
        self.index_sender.send((key, offset, size)).map_err(|e| {
            box_err!("send err: {}", e)
        })
    }


    pub fn write_needle(&mut self, n: &mut Needle) -> Result<u32> {
        if self.read_only {
            return Err(box_err!("{} is read-only", self.data_file_name()));
        }

        let mut offset: u64;
        let size: u32;
        let version = self.version();

        {
            let file = self.file_mut();
            offset = file.seek(SeekFrom::End(0))?;

            if offset % NEEDLE_PADDING_SIZE as u64 != 0 {
                offset = offset +
                    (NEEDLE_PADDING_SIZE as u64 - offset % NEEDLE_PADDING_SIZE as u64);
                offset = file.seek(SeekFrom::Start(offset))?;
            }

            size = match n.append(file, version) {
                Ok(s) => s.0,
                Err(err) => {
                    // TODO
                    // truncate file
                    return Err(err);
                }
            };
        }

        offset = offset / NEEDLE_PADDING_SIZE as u64;
        self.nm.set(
            n.id,
            NeedleValue {
                offset: offset as u32,
                size: n.size,
            },
        );

        self.async_wirte_index_file(n.id, offset as u32, n.size)?;


        if self.last_modified_time < n.last_modified {
            self.last_modified_time = n.last_modified;
        }

        Ok(size)

    }

    pub fn delete_needle(&mut self, n: &Needle) -> Result<u32> {
        let _ = n;
        panic!("TODO");
    }

    pub fn read_needle(&mut self, n: &mut Needle) -> Result<u32> {
        let nv = self.nm.get(n.id).ok_or::<Error>(box_err!("Not Found"))?;
        debug!("read needle: {:?}", nv);

        if nv.offset == 0 {
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

    pub fn data_file_name(&self) -> String {
        let mut f = self.file_name();
        f.push_str(".dat");
        f
    }

    pub fn index_file_name(&self) -> String {
        let mut f = self.file_name();
        f.push_str(".idx");
        f
    }

    pub fn file_name(&self) -> String {
        let mut rt = self.dir.clone();
        if !rt.ends_with("/") {
            rt.push_str("/");
        }
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
            delete_byte_count: self.nm.deleted_byte_count(),
            read_only: self.read_only,
        }
    }

    pub fn destroy(&mut self) -> Result<()> {
        if self.read_only {
            return Err(box_err!("{} is read only", self.data_file_name()));
        }

        fs::remove_file(Path::new(&self.data_file_name()))?;

        self.nm.destroy()?;

        Ok(())
    }

    pub fn content_size(&self) -> u64 {
        // TODO
        0
    }

    pub fn size(&self) -> Result<u64> {
        match self.data_file {
            None => return Err(box_err!("not open {}", self.data_file_name())),
            Some(ref file) => Ok(file.metadata()?.len()),
        }
    }

    // volume is expired if modified time + volume ttl < now
    // except when volume is empty
    // or when the volume does not have a ttl
    // or when volumeSizeLimit is 0 when server just starts
    pub fn expired(&self, volume_size_limit: u64) -> bool {
        if volume_size_limit == 0 {
            return false;
        }

        if self.content_size() == 0 {
            return false;
        }

        // change self ttl to option?
        if self.super_block.ttl.minutes() == 0 {
            return false;
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        if now.as_secs() > self.last_modified_time + self.super_block.ttl.minutes() as u64 * 60 {
            return true;
        }

        false
    }

    pub fn need_to_replicate(&self) -> bool {
        self.replica_placement.get_copy_count() > 1
    }

    // wait either maxDelayMinutes or 10% of ttl minutes
    pub fn expired_long_enough(&self, max_delay_minutes: u64) -> bool {
        let ttl = self.super_block.ttl;
        if ttl.minutes() == 0 {
            return false;
        }

        let mut delay: u64 = ttl.minutes() as u64 / 10;
        if delay > max_delay_minutes {
            delay = max_delay_minutes;
        }

        if (ttl.minutes() as u64 + delay) * 60 + self.last_modified_time <
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        {
            return true;
        }

        false
    }
}
