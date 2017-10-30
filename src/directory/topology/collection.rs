use std::collections::HashMap;
use std::ops::Add;
use std::sync::Arc;
use std::cell::RefCell;


use super::{VolumeLayout, DataNode};
use storage::{ReplicaPlacement, TTL, VolumeId};

#[derive(Debug)]
pub struct Collection {
    pub name: String,
    pub volume_size_limit: u64,
    pub type2layout: HashMap<String, VolumeLayout>,
}



impl Collection {
    pub fn new(name: &str, volume_size_limit: u64) -> Collection {
        Collection {
            name: String::from(name),
            volume_size_limit: volume_size_limit,
            type2layout: HashMap::new(),
        }
    }


    pub fn get_or_create_volume_layout(&mut self, rp: ReplicaPlacement, ttl: Option<TTL>) -> &mut VolumeLayout {

        let mut key = rp.string();
        if ttl.is_some() {
            key = key.add(&ttl.unwrap().string());
        }

        let vsize = self.volume_size_limit;

        self.type2layout
            .entry(key)
            .or_insert_with(|| VolumeLayout::new(rp, ttl, vsize))

    }

    pub fn lookup(&self, vid: VolumeId) -> Option<Vec<Arc<RefCell<DataNode>>>> {
        for (_, layout) in &self.type2layout {
            let ret = layout.lookup(vid);
            if ret.is_some() {
                return ret;
            }
        }

        None
    }
}


