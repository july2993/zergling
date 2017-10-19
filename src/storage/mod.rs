mod errors;
mod needle;
mod version;
mod volume;
mod replica_placement;
mod ttl;
mod needle_value;
mod volume_info;
mod server;

pub use self::version::{Version, CurrentVersion};
pub use self::errors::{Error, Result};
pub use self::ttl::{TTL, Unit};
pub use self::replica_placement::ReplicaPlacement;
pub use self::needle_value::NeedleValue;
pub use self::volume_info::VolumeInfo;
pub use self::server::Server;


pub type VolumeId = u32;
