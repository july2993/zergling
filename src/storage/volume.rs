
use std::fs::File;
use std::io::prelude::*;

use super::Version;
use super::ReplicaPlacement;
use super::TTL;
use super::VolumeId;


#[derive(Debug)]
pub struct SuperBlock {
    pub Version: Version,
    pub ReplicaPlacement: ReplicaPlacement,
    pub TTL: TTL,
    pub CompactRevision: u16,
}


#[derive(Debug)]
pub struct Volume {
    pub id: VolumeId,
    pub Dir: String,
    pub Collection: String,
    pub DataFile: File,
    // pub NM: NeedleMapper,
    
    pub ReadOnly: bool,

    pub SuperBlock: SuperBlock,

    pub LastModifiedTime: u64,

    pub LastCompactIndexOffset: u64,
    pub LastCompactRevision: u16,
}
