// TODO remove allow
#![allow(dead_code)]

use std::fs::File;

use super::Version;
use super::ReplicaPlacement;
use super::TTL;
use super::{VolumeId, Result, Error, Needle, NeedleMapper, NeedleValueMap, NeedleMapType};

use super::needle::TOMBSTONE_FILE_SIZE;
use super::needle;
use time;



#[derive(Debug, Default)]
pub struct SuperBlock {
    pub version: Version,
    pub replica_placement: ReplicaPlacement,
    pub ttl: TTL,
    pub compact_revision: u16,
}


pub struct Volume {
    pub id: VolumeId,
    pub dir: String,
    pub collection: String,
    pub data_file: File,
    pub nm: NeedleMapper,
    pub needle_map_kind: NeedleMapType,

    pub read_only: bool,

    pub file: File,

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

        Err(box_err!("todo"))
        // let mut v = Volume {
        //     dir: dir.to_string(),
        //     collection: collection.to_string(),
        //     id: id,
        //     super_block: sb,
        //     needle_map_kind: needle_map_kind,
        // };

        // v.load()?;
        // v
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
            &mut self.data_file,
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

    pub fn size() -> i64 {
        panic!("TODO");
        0
    }
}
