use directory::sequencer::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;

use super::{Collection, DataCenter, DataNode, VolumeGrowOption, VolumeLayout};
use directory::Result;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeStruct;
use storage;
use storage::VolumeId;
use rand;



#[derive(Debug, Clone)]
pub struct Topology {
    pub sequence: MemorySequencer,
    pub collection_map: HashMap<String, Collection>,
    pub pulse: u64,
    pub volume_size_limit: u64,
    pub data_centers: HashMap<String, Arc<RefCell<DataCenter>>>,
}


impl Serialize for Topology {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Topology", 4)?;
        state.serialize_field("collection_map", &self.collection_map)?;
        state.serialize_field("pulse", &self.pulse)?;
        state.serialize_field("volume_size_limit", &self.volume_size_limit)?;

        let mut nodes = vec![];
        for (_, n) in self.data_centers.iter() {
            nodes.push(n.borrow().clone());
        }
        state.serialize_field("dataCenters", &nodes)?;

        state.end()
    }
}

unsafe impl Send for Topology {}

impl Topology {
    pub fn new(seq: MemorySequencer, volume_size_limit: u64, pulse: u64) -> Topology {
        Topology {
            sequence: seq,
            collection_map: HashMap::new(),
            pulse: pulse,
            volume_size_limit: volume_size_limit,
            data_centers: HashMap::new(),
        }
    }



    pub fn get_or_create_data_center(&mut self, name: &str) -> Arc<RefCell<DataCenter>> {
        self.data_centers
            .entry(String::from(name))
            .or_insert(Arc::new(RefCell::new(DataCenter::new(name))))
            .clone()
    }

    pub fn lookup(
        &mut self,
        collection: String,
        vid: VolumeId,
    ) -> Option<Vec<Arc<RefCell<DataNode>>>> {
        if collection.is_empty() {
            for (_name, c) in &self.collection_map {
                let rt = c.lookup(vid);
                if rt.is_some() {
                    return rt;
                }
            }
        } else {
            match self.collection_map.get(&collection) {
                Some(c) => {
                    let rt = c.lookup(vid);
                    if rt.is_some() {
                        return rt;
                    }
                }
                None => (),
            };
        }

        None
    }

    fn get_volume_layout(
        &mut self,
        collection: &str,
        rp: storage::ReplicaPlacement,
        ttl: storage::TTL,
    ) -> &mut VolumeLayout {
        self.collection_map
            .entry(String::from(collection))
            .or_insert(Collection::new(collection, self.volume_size_limit))
            .get_or_create_volume_layout(rp, Some(ttl))
    }


    pub fn has_writable_volume(&mut self, option: &VolumeGrowOption) -> bool {
        let vl = self.get_volume_layout(&option.collection, option.replica_placement, option.ttl);

        vl.get_active_volume_count(option) > 0
    }

    // free volume
    pub fn free_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, dc) in &self.data_centers {
            let dc_ref = dc.borrow();
            ret += dc_ref.max_volumes() - dc_ref.has_volumes();
        }
        ret
    }

    //@return (file_id, count, datanode)
    pub fn pick_for_write(
        &mut self,
        count: u64,
        option: &VolumeGrowOption,
    ) -> Result<(String, u64, Arc<RefCell<DataNode>>)> {
        let ret: (VolumeId, Vec<Arc<RefCell<DataNode>>>);
        {
            let layout =
                self.get_volume_layout(&option.collection, option.replica_placement, option.ttl);
            ret = layout.pick_for_write(&option)?;
        }

        let (file_id, c) = self.sequence.next_file_id(count);


        let file_id = storage::FileID {
            volume_id: ret.0,
            key: file_id,
            hash_code: rand::random::<u32>(),
        }.string();

        Ok((file_id, c, ret.1[0].clone()))
    }

    pub fn register_volume_layout(&mut self, vi: storage::VolumeInfo, dn: Arc<RefCell<DataNode>>) {
        self.get_volume_layout(&vi.collection, vi.replica_placement, vi.ttl)
            .register_volume(&vi, dn);
    }

    pub fn un_register_volume_layout(
        &mut self,
        vi: storage::VolumeInfo,
        dn: Arc<RefCell<DataNode>>,
    ) {
        self.get_volume_layout(&vi.collection, vi.replica_placement, vi.ttl)
            .un_register_volume(&vi, dn);
    }

    pub fn get_max_volume_id(&self) -> VolumeId {
        let mut vid: VolumeId = 0;
        for (_, dc) in self.data_centers.iter() {
            let other = dc.borrow().max_volume_id;
            if other > vid {
                vid = other;
            }
        }

        vid
    }

    pub fn next_volume_id(&mut self) -> VolumeId {
        let vid = self.get_max_volume_id();

        vid + 1
    }
}
