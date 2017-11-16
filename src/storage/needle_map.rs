use std;
use std::io::BufReader;
use std::fs::File;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};
use byteorder::ByteOrder;
use std::io::prelude::*;
use byteorder::WriteBytesExt;
use std::io::SeekFrom;
use storage::{needle, volume};


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

    pub fn load_idx_file(&mut self, index_file: &mut File, data_file: &mut File) -> Result<()> {
        let mut last_offset = 0;
        let mut last_size = 0;
        walk_index_file(index_file, |key, offset, size| -> Result<()> {
            if offset > last_offset {
                last_offset = offset;
                last_size = size;
            }

            if offset > 0 {
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
        })?;


        // load the left needle has not write in index file
        let mut next_offset;
        if last_offset > 0 {
            next_offset = needle::true_offset(last_offset) + needle::get_actual_size(last_size);
        } else {
            next_offset = volume::SUPER_BLOCK_SIZE as u64;
        }

        // TODO change magic number(maybe redesign needle format, header should include flag
        // need flag,size,id
        let mut bytes: Vec<u8> = vec![0; 21];
        while let Ok(_) = data_file.seek(SeekFrom::Start(next_offset)) {
            if data_file.read_exact(&mut bytes).is_err() {
                debug!("read exact fail");
                break;
            }

            let key = BigEndian::read_u64(
                &bytes[needle::NEEDLE_ID_OFFSET..needle::NEEDLE_ID_OFFSET + 8],
            );
            let size = BigEndian::read_u32(
                &bytes[needle::NEEDLE_SIZE_OFFSET..needle::NEEDLE_SIZE_OFFSET + 4],
            );
            let flag = bytes[needle::NEEDLE_FLAG_OFFSET];
            if flag & needle::FLAG_IS_DELETE > 0 {
                self.delete(key);
            } else {
                let offset = next_offset / needle::NEEDLE_PADDING_SIZE as u64;
                self.set(
                    key,
                    NeedleValue {
                        offset: offset as u32,
                        size: size,
                    },
                );
            }

            // write to index file
            let mut buf: Vec<u8> = vec![];
            buf.write_u64::<BigEndian>(key).unwrap();
            if flag & needle::FLAG_IS_DELETE > 0 {
                buf.write_u32::<BigEndian>(0).unwrap();
            } else {
                let offset = next_offset / needle::NEEDLE_PADDING_SIZE as u64;
                buf.write_u32::<BigEndian>(offset as u32).unwrap();
            }
            buf.write_u32::<BigEndian>(size).unwrap();

            next_offset += needle::get_actual_size(size);
        }

        Ok(())
    }

    pub fn set(&mut self, key: u64, needle_value: NeedleValue) -> Option<NeedleValue> {
        debug!("needle map set key: {}, {:?}", key, needle_value);
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

        debug!("needle map delete key: {} {:?}", key, old);
        old
    }

    pub fn get(&self, key: u64) -> Option<NeedleValue> {
        let rt = self.nvp.get(key);
        debug!("needle map get key: {} {:?}", key, rt);

        rt
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
