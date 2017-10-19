use directory::topology::{Topology, DataNode, VolumeInfo};
use directory::{Result,Error};
use storage;
use storage::VolumeId;
use std::rc::Rc;

use util;

use serde_json;
use serde_json::Value;


#[derive(Debug)]
pub struct VolumeGrow {

}

impl VolumeGrow {
    pub fn new() -> VolumeGrow {
        VolumeGrow{}
    }

    pub fn grow_by_type(&self, option: &VolumeGrowOption, topo: &Topology) -> Result<i64> {
        let count = self.logicalCount(option.replica_placement.get_copy_count());
        self.grow_by_count_and_type(count, option, topo)
    }

    fn logicalCount(&self, pcount: i64) -> i64 {
        match pcount {
            1 => 7,
            2 => 6,
            3 => 3,
            _ => 1,
        }
    }

    fn find_empty_slots(&self, option: VolumeGrowOption, topo: Topology) -> Result<i64> {
        panic!("todo");
        Ok(1)
    }

    fn find_and_grow(&self, option: &VolumeGrowOption, topo: &Topology) -> Result<i64> {
        panic!("todo");

        Ok(1)
    }

    fn grow_by_count_and_type(&self, count: i64, option: &VolumeGrowOption, topo: &Topology) -> Result<i64> {
        let mut grow_count = 0;
        for i in 0..count {
            match self.find_and_grow(option, topo) {
                Ok(v) => grow_count += v,
                Err(err) => {
                    // TODO return err?
                    break;
                }
            }
        }

        Ok(grow_count)
    }

    fn grow(&self, vid: VolumeId, option: &VolumeGrowOption, topo: &Topology, nodes: Vec<Rc<DataNode>>) -> Result<()> {
        for nd in nodes {
            // let a: u8 = nd;
            allocate_volume(&nd, vid, option)?;
            let volumeInfo = VolumeInfo {
                id: vid,
                size: 0,
                collection: option.collection.clone(),
                replica_placement: option.replica_placement,
                ttl: option.ttl,
                version: storage::CurrentVersion,
                ..Default::default()
            };
            // TODO init
            // topo.registerVolumeLayout(vi, nd)
             
            // let mut tmp = nd.clone();
            // tmp.add_or_update_volume(volumeInfo)
        }
        Ok(())
    }

}


#[derive(Debug,Default)]
pub struct VolumeGrowOption {
    pub collection: String,
    pub replica_placement: storage::ReplicaPlacement,
    pub ttl: storage::TTL,
    pub preallocate: i64,
    pub DataCenter: String,
    pub Rack: String,
    pub DataNode: String,
}

impl VolumeGrowOption {
    // fn new() -> VolumeGrowOption {
    //     panic!("todo");
    //     VolumeGrowOption{}
    // }
}


fn allocate_volume(dn: &DataNode, vid: VolumeId, option: &VolumeGrowOption) -> Result<()> {
    
    let vid_p = &vid.to_string();
    let collection_p = &option.collection;
    let rp_p = &option.replica_placement.string();
    let ttl_p = &option.ttl.string();
    let pre_p = &format!("{}", option.preallocate);

    let mut params: Vec<(&str, &str)> = vec![];
    
    params.push(("volume", vid_p));
    params.push(("collection", collection_p));
    params.push(("replication", rp_p));
    params.push(("ttl", ttl_p));
    params.push(("preallocate", pre_p));

    let body = util::post(&format!("http://{}/admin/assign_volume", dn.url()), &params)?;
    
    let v: serde_json::Value = serde_json::from_slice(&body)?;

    if let Value::String(ref s) = v["Error"] {
        return Err(Error::from(s.clone()));
    }
    

    Ok(())
}
