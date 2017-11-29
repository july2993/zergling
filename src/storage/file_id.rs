use super::VolumeId;
use byteorder::BigEndian;
use byteorder::WriteBytesExt;


pub struct FileID {
    pub volume_id: VolumeId,
    pub key: u64,
    pub hash_code: u32,
}



impl FileID {
    pub fn string(&self) -> String {
        format!("{},{:x}{:08x}", self.volume_id, self.key, self.hash_code)
    }
}
