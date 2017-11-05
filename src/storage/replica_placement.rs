
use super::Result;
use std::fmt::Write;
use storage::errors::Error::ParseReplicaPlacement;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ReplicaPlacement {
    pub same_rack_count: u8,
    pub diff_rack_count: u8,
    pub diff_data_center_count: u8,
}


impl ReplicaPlacement {
    pub fn from_u8(u: u8) -> Result<ReplicaPlacement> {
        let s = format!("{:03}", u);
        ReplicaPlacement::new(&s)
    }

    pub fn byte(&self) -> u8 {
        self.diff_data_center_count * 100 + self.diff_rack_count * 10 + self.same_rack_count
    }

    pub fn new(s: &str) -> Result<ReplicaPlacement> {
        if s.len() != 3 {
            return Err(ParseReplicaPlacement(String::from(s)));
        }

        let bytes = s.as_bytes();

        let rp = ReplicaPlacement {
            same_rack_count: bytes[0] - '0' as u8,
            diff_rack_count: bytes[1] - '0' as u8,
            diff_data_center_count: bytes[1] - '0' as u8,
        };

        Ok(rp)
    }

    pub fn string(&self) -> String {
        let mut s = String::new();
        // should never fail
        write!(
            s,
            "{}{}{}",
            self.diff_data_center_count,
            self.diff_rack_count,
            self.same_rack_count
        ).unwrap();

        s
    }

    pub fn get_copy_count(&self) -> i64 {
        (self.diff_data_center_count + self.diff_rack_count + self.same_rack_count + 1) as i64
    }
}
