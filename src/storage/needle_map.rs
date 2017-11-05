use storage;
use std;
use std::io::BufReader;
use std::fs::{File, metadata, Metadata};
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::prelude::*;
use storage::needle::TOMBSTONE_FILE_SIZE;

use storage::{NeedleValueMap, NeedleValue, Result};
use storage::needle_value_map::MemNeedleValueMap;


#[derive(Copy, Clone)]
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


// #[derive(Default)]
pub struct NeedleMapper {
    nvp: Box<NeedleValueMap>,
}

impl std::default::Default for NeedleMapper {
    fn default() -> Self {
        NeedleMapper { nvp: Box::new(MemNeedleValueMap::default()) }
    }
}

impl NeedleMapper {
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
        self.nvp.set(key, needle_value)
    }

    pub fn delete(&mut self, key: u64) -> Option<NeedleValue> {
        self.nvp.delete(key)
    }

    pub fn get(&self, key: u64) -> Option<NeedleValue> {
        self.nvp.get(key)
    }

    pub fn file_count(&self) -> u64 {
        0
    }
    pub fn delete_count(&self) -> u64 {
        0
    }
    pub fn delete_byte_count(&self) -> u64 {
        0
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


pub struct BaseNeedleMapper {
    index_file: File,
    deletion_counter: u64,
    file_counter: u64,
    deletion_byte_counter: u64,
    file_byte_counter: u64,
    maximum_file_key: u64,
}
