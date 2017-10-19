
use storage::{VolumeId, ReplicaPlacement, Version, TTL};



#[derive(Debug, Default)]
pub struct VolumeInfo {
    pub id: VolumeId,
    pub size: u64,
    pub replica_placement: ReplicaPlacement,
    pub ttl: TTL,
    pub collection: String,
    pub version: Version,
    pub file_count: i64,
    pub delete_count: i64,
    pub delte_byte_count: u64,
    pub read_only: bool,
}


impl VolumeInfo {
    // fn new() -> VolumeInfo {
    //     VolumeInfo{}
    // }
}
