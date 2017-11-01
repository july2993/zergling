use storage;
use std::collections::HashMap;

use storage::{VolumeId, Volume, NeedleMapType, Result};




pub struct DiskLocation {
    pub directory: String,
    pub max_volume_count: i64,
    pub volumes: HashMap<VolumeId, Volume>,
}


impl DiskLocation {
    pub fn new(dir: &str, max_volume_count: i64) -> DiskLocation {
        DiskLocation {
            directory: String::from(dir),
            max_volume_count: max_volume_count,
            volumes: HashMap::new(),
        }
    }

    pub fn concurrent_loading_volumes(&mut self, needle_map_kind: NeedleMapType, concurrent: bool) {
        panic!("TODO");
    }

    pub fn load_existing_volumes(&mut self, needle_map_kind: NeedleMapType) {
        self.concurrent_loading_volumes(needle_map_kind, true);
    }

    pub fn delete_volume(&mut self, vid: VolumeId) -> Result<()> {
        panic!("TODO");

        Ok(())
    }
}
