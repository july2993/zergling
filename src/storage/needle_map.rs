use std;
use std::io::BufReader;
use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use storage::needle::TOMBSTONE_FILE_SIZE;

use storage::{NeedleValueMap, NeedleValue, Result};
use storage::needle_value_map::MemNeedleValueMap;


#[derive(Copy, Clone, Debug)]
pub enum NeedleMapType {
    NeedleMapInMemory = 0,
    // NeedleMapLevelDb = 1,
    // NeedleMapBoltDb = 2,
    // NeedleMapBtree = 3,
}

impl Default for NeedleMapType {
    fn default() -> Self {
        NeedleMapType::NeedleMapInMemory
    }
}

#[derive(Default)]
struct Metric {
    maximum_file_key: u64,
    file_count: u64,
    deleted_count: u64,
    deleted_byte_count: u64,
    file_byte_count: u64,
}

// #[derive(Default)]
pub struct NeedleMapper {
    nvp: Box<NeedleValueMap>,

    metric: Metric,
}

impl std::default::Default for NeedleMapper {
    fn default() -> Self {
        NeedleMapper {
            nvp: Box::new(MemNeedleValueMap::new()),
            metric: Metric::default(),
        }
    }
}

impl NeedleMapper {
    pub fn new(kind: NeedleMapType) -> NeedleMapper {
        #[allow(unreachable_patterns)]
        match kind {
            NeedleMapType::NeedleMapInMemory => {
                NeedleMapper {
                    nvp: Box::new(MemNeedleValueMap::new()),
                    metric: Metric::default(),
                }
            }
            _ => panic!("not support map type: {:?}", kind),
        }
    }

    pub fn load_idx_file(&mut self, f: &mut File) -> Result<()> {
        walk_index_file(f, |key, offset, size| -> Result<()> {
            if offset > 0 && size != TOMBSTONE_FILE_SIZE {
                self.set(
                    key,
                    NeedleValue {
                        offset: offset,
                        size: size,
                    },
                );
            } else {
                self.delete(key);
            }
            Ok(())
        })
    }

    pub fn set(&mut self, key: u64, needle_value: NeedleValue) -> Option<NeedleValue> {
        if key > self.metric.maximum_file_key {
            self.metric.maximum_file_key = key;
        }
        self.metric.file_count += 1;
        self.metric.file_byte_count += needle_value.size as u64;
        let old = self.nvp.set(key, needle_value);

        if let Some(n) = old {
            self.metric.deleted_count += 1;
            self.metric.deleted_byte_count += n.size as u64;
        }

        old
    }

    pub fn delete(&mut self, key: u64) -> Option<NeedleValue> {
        let old = self.nvp.delete(key);

        if let Some(n) = old {
            self.metric.deleted_count += 1;
            self.metric.deleted_byte_count += n.size as u64;
        }

        old
    }

    pub fn get(&self, key: u64) -> Option<NeedleValue> {
        self.nvp.get(key)
    }

    pub fn destroy(&mut self) -> Result<()> {
        // TODO may need rm index file


        Ok(())
    }

    pub fn file_count(&self) -> u64 {
        self.metric.file_count
    }
    pub fn delete_count(&self) -> u64 {
        self.metric.deleted_count
    }
    pub fn deleted_byte_count(&self) -> u64 {
        self.metric.deleted_byte_count
    }

    pub fn max_file_key(&self) -> u64 {
        self.metric.maximum_file_key
    }

    pub fn content_size(&self) -> u64 {
        self.metric.file_byte_count
    }
}

fn idx_entry(buf: &[u8]) -> (u64, u32, u32) {
    let mut rdr = Cursor::new(buf);
    let key = rdr.read_u64::<BigEndian>().unwrap();
    let offset = rdr.read_u32::<BigEndian>().unwrap();
    let size = rdr.read_u32::<BigEndian>().unwrap();


    (key, offset, size)
}



// walks through index file, call fn(key, offset, size), stop with error returned by fn
pub fn walk_index_file<T>(f: &mut File, mut call: T) -> Result<()>
where
    T: FnMut(u64, u32, u32) -> Result<()>,
{
    let mut reader = BufReader::new(f.try_clone()?);
    let mut buf: Vec<u8> = vec![0; 16];


    // if there is a not complete entry, whill err
    for _ in 0..(f.metadata().unwrap().len() + 15) / 16 {
        reader.read_exact(&mut buf)?;

        let (key, offset, size) = idx_entry(&buf);
        call(key, offset, size)?;
    }

    Ok(())
}
