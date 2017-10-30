mod errors;
mod needle;
mod version;
mod volume;
mod replica_placement;
mod ttl;
mod needle_value;
mod volume_info;
mod file_id;
mod server;

pub use self::version::{Version, CURRENT_VERSION};
pub use self::errors::{Error, Result};
pub use self::ttl::{TTL, Unit};
pub use self::replica_placement::ReplicaPlacement;
pub use self::needle_value::NeedleValue;
pub use self::volume_info::VolumeInfo;
pub use self::server::Server;
pub use self::file_id::FileID;

// use std::string::ToString;

pub type VolumeId = u32;

// impl VolumeId {
// }


// impl ToString for VolumeId {
//     fn to_string(&self) -> String {
//         self as u32.to_string()
//     }
// }
