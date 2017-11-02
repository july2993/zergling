use storage;
use std::fs::File;

use storage::{NeedleValueMap, NeedleValue};


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


// #[derive(Debug)]
pub struct NeedleMapper {
    nvp: Box<NeedleValueMap>,
}

impl NeedleMapper {
    pub fn set(&mut self, key: u64, needle_value: NeedleValue) -> Option<NeedleValue> {
        self.nvp.set(key, needle_value)
    }

    pub fn delete(&mut self, key: u64) -> Option<NeedleValue> {
        self.nvp.delete(key)
    }

    pub fn get(&self, key: u64) -> Option<NeedleValue> {
        self.nvp.get(key)
    }
}


pub struct BaseNeedleMapper {
    index_file: File,
    deletion_counter: u64,
    file_counter: u64,
    deletion_byte_counter: u64,
    file_byte_counter: u64,
    maximum_file_key: u64,
}
