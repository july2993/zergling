mod errors;
mod needle;
mod version;
mod replica_placement;
mod ttl;
mod needle_value;
mod needle_value_map;
mod needle_map;
mod volume_info;
mod file_id;
mod server;
mod store;
mod disk_location;
mod volume;
mod api;

pub use self::version::{Version, CURRENT_VERSION};
pub use self::errors::{Error, Result};
pub use self::ttl::{TTL, Unit};
pub use self::replica_placement::ReplicaPlacement;
pub use self::needle_value::NeedleValue;
pub use self::needle_map::*;
pub use self::volume_info::VolumeInfo;
pub use self::server::Server;
pub use self::file_id::FileID;
pub use self::store::Store;
pub use self::disk_location::DiskLocation;
pub use self::volume::Volume;
pub use self::needle::Needle;
pub use self::needle_value_map::NeedleValueMap;

// use std::string::ToString;

pub type VolumeId = u32;

// impl VolumeId {
// }


// impl ToString for VolumeId {
//     fn to_string(&self) -> String {
//         self.to_string()
//     }
// }
