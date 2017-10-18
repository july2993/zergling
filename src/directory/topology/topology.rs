

use directory::sequencer::*;
use std::collections::HashMap;
use std::sync::mpsc;
use std::marker::Send;

use super::{DataCenter, Rack, DataNode, Collection, VolumeLayout, VolumeGrowOption};
use directory::{Error, Result};
use storage;
use storage::{VolumeId};



#[derive(Debug)]
pub struct Topology {
    pub sequence: MemorySequencer,

    pub collectionMap: HashMap<String, Collection>,

    pub pulse: u64,

    pub volumeSizeLimit: u64,

    pub max_volume_id: VolumeId,


    pub data_centers: HashMap<String, DataCenter>,

}


impl Topology {
    pub fn new(seq: MemorySequencer, volumeSizeLimit: u64, pulse: u64) -> Topology {
        Topology {
            sequence: seq,
            collectionMap: HashMap::new(),
            pulse: pulse,
            volumeSizeLimit: volumeSizeLimit,
            data_centers: HashMap::new(),
            max_volume_id: 0,
        }
    }


    pub fn get_or_create_datacenter(&mut self, name: &str) ->  &mut DataCenter {
        self.data_centers
            .entry(String::from(name))
            .or_insert_with(|| DataCenter::new(name) )
    }

    pub fn lookup(&mut self, collection: String, vid: VolumeId) -> Option<Vec<DataNode>> {
        if collection.is_empty() {
            for (name, c) in &self.collectionMap {
                let rt = c.lookup(vid);
                if rt.is_some() {
                    return rt;
                }
            }
        } else {
            match self.collectionMap.get(&collection) {
                Some(c) => {
                    let rt = c.lookup(vid);
                    if rt.is_some() {
                        return rt;
                    }
                },
                None => (),
            };
        }

        None
    }

    fn get_volume_layout(&mut self, collection: &str, rp: storage::ReplicaPlacement, ttl: storage::TTL) -> &mut VolumeLayout {
        self.collectionMap
            .entry(String::from(collection))
            .or_insert(Collection::new(collection, self.volumeSizeLimit))
            .get_or_create_volume_layout(rp, Some(ttl))
    }


    pub fn has_writable_volume(&mut self, option: &VolumeGrowOption) -> bool {
        let vl = self.get_volume_layout(&option.collection, option.replica_placement, option.ttl);

        vl.get_active_volume_count(option) > 0
    }

    // free volume
    pub fn free_space(&self) -> i64 {
        panic!("todo");
        0
    }

    pub fn pick_for_write(&self, count: i64, option: &VolumeGrowOption) -> Result<(String, i64, DataNode)> {
        panic!("todo");

        Ok((String::from("ok"), 0, DataNode::default()))
    }

}
