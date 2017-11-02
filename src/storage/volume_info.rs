
use storage::{VolumeId, ReplicaPlacement, Version, TTL, Result};
use pb;



#[derive(Clone, Debug, Default)]
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
    pub fn new(m: &pb::zergling::VolumeInformationMessage) -> Result<VolumeInfo> {
        let rp = ReplicaPlacement::from_u8(m.replica_placement as u8)?;
        Ok(VolumeInfo {
            id: m.id as VolumeId,
            size: m.size,
            collection: m.collection.clone(),
            file_count: m.file_count as i64,
            delete_count: m.delete_count as i64,
            delte_byte_count: m.deleted_byte_count,
            read_only: m.read_only,
            version: m.version as Version,
            ttl: TTL::from(m.ttl),
            replica_placement: rp,
        })
    }
}
