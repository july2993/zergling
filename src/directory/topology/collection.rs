use std::fmt::Write;
use std::collections::HashMap;
use std::ops::Add;


use super::{VolumeLayout, DataNode};
use storage::{ReplicaPlacement, TTL, VolumeId};

#[derive(Debug)]
pub struct Collection {
    pub name: String,
    pub volumeSizeLimit: u64,
    pub type2layout: HashMap<String, VolumeLayout>,
}



impl Collection {
    pub fn new(name: &str, volumeSizeLimit: u64) -> Collection {
        Collection {
            name: String::from(name),
            volumeSizeLimit: volumeSizeLimit,
            type2layout: HashMap::new(),
        }
    }


    pub fn get_or_create_volume_layout(&mut self, rp: ReplicaPlacement, ttl: Option<TTL>) -> &mut VolumeLayout {

        let mut key = rp.string();
        if ttl.is_some() {
            key = key.add(&ttl.unwrap().string());
        }

        let vsize = self.volumeSizeLimit;

        self.type2layout
            .entry(key)
            .or_insert_with(|| VolumeLayout::new(rp, ttl, vsize))

    }

    pub fn lookup(&self, vid: VolumeId) -> Option<Vec<DataNode>> {
        panic!("todo");
        None
    }
}


