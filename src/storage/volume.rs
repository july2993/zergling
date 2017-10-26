
use std::fs::File;

use super::Version;
use super::ReplicaPlacement;
use super::TTL;
use super::VolumeId;


#[derive(Debug)]
pub struct SuperBlock {
    pub version: Version,
    pub replica_placement: ReplicaPlacement,
    pub ttl: TTL,
    pub compact_revision: u16,
}


#[derive(Debug)]
pub struct Volume {
    pub id: VolumeId,
    pub dir: String,
    pub collection: String,
    pub data_file: File,
    // pub NM: NeedleMapper,
    
    pub readonly: bool,

    pub super_block: SuperBlock,

    pub last_modified_time: u64,

    pub last_compact_index_offset: u64,
    pub last_compact_revision: u16,
}
