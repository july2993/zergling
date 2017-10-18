

use directory::topology::Topology;
use directory::{Result};
use storage;


#[derive(Debug)]
pub struct VolumeGrow {

}

impl VolumeGrow {
    pub fn new() -> VolumeGrow {
        panic!("todo");
        VolumeGrow{}
    }

    pub fn grow_by_type(&self, option: &VolumeGrowOption, topo: &Topology) -> Result<i64> {
        panic!("todo");
        Ok(1)

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
