
use super::{Error, Result};
use std::fmt::Write;
use storage::errors::Error::ParseReplicaPlacement;

#[derive(Debug, Default, Copy, Clone)]
pub struct ReplicaPlacement {
    pub SameRackCount: u8,
    pub DiffRackCount: u8,
    pub DiffDataCenterCount: u8,
}

impl ReplicaPlacement {
    pub fn new(s: &str) -> Result<ReplicaPlacement> {
        if s.len() != 3 {
            return Err(ParseReplicaPlacement(String::from(s)));
        }

        let bytes = s.as_bytes();

        let rp = ReplicaPlacement {
            SameRackCount: bytes[0] - '0' as u8,
            DiffRackCount: bytes[1] - '0' as u8, 
            DiffDataCenterCount: bytes[1] - '0' as u8,
        };

        Ok(rp)
    }

    pub fn string(&self) -> String {
        let mut s = String::new();
        write!(s, "{}{}{}", self.DiffDataCenterCount, self.DiffRackCount, self.SameRackCount);

        s
    }

    pub fn get_copy_count(&self) -> i64 {
        (self.DiffDataCenterCount + self.DiffRackCount + self.SameRackCount + 1) as i64
    }
}
