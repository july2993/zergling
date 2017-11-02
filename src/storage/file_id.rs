
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
        let mut bytes: Vec<u8> = vec![];

        bytes.write_u64::<BigEndian>(self.key).unwrap();
        bytes.write_u32::<BigEndian>(self.hash_code).unwrap();


        let mut nonzero_index = 0;
        for i in 0..12 {
            if bytes[i] != 0 {
                nonzero_index = i;
                break;
            }
        }

        let mut ret = self.volume_id.to_string();

        let idx = ret.len();
        ret.insert(idx, ',');

        for idx in nonzero_index..12 {
            let hex = format!("{:x}", bytes[idx]);
            let idx = ret.len();
            ret.insert_str(idx, &hex);
        }

        ret
    }
}
