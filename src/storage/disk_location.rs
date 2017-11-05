use storage;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::path::Path;


use storage::{VolumeId, Volume, NeedleMapType, Result, ReplicaPlacement, TTL};




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


    pub fn volume_id_from_path(&self, p: &Path) -> Result<(VolumeId, String)> {
        if p.is_dir() || !p.ends_with(".dat") {
            return Err(box_err!("not valid file: {}", p.to_str().unwrap()));
        }

        let name = p.file_name().unwrap().to_str().unwrap();

        let mut collection: &str;
        let mut id: &str;
        if let Some(idx) = name.find("_") {
            collection = &name[0..idx];
            id = &name[idx + 1..name.len() - 4];
        } else {
            collection = &name[0..0];
            id = &name[0..name.len() - 4];
        }

        let vid = id.parse()?;

        Ok((vid, collection.to_string()))
    }

    pub fn load_existing_volumes(&mut self, needle_map_kind: NeedleMapType) -> Result<()> {
        // TODO concurrent load volumes
        // self.concurrent_loading_volumes(needle_map_kind, true);
        let dir = Path::new(&self.directory);
        for entry in fs::read_dir(dir)? {
            let file = entry?.path();
            let fpath = file.as_path();

            if fpath.ends_with(".dat") {
                match self.volume_id_from_path(fpath) {
                    Ok((vid, collection)) => {
                        if self.volumes.get(&vid).is_some() {
                            continue;
                        }
                        let vr = Volume::new(
                            &self.directory,
                            &collection,
                            vid,
                            needle_map_kind,
                            ReplicaPlacement::default(),
                            TTL::default(),
                            0,
                        );
                        match vr {
                            Ok(v) => {
                                self.volumes.insert(vid, v);
                            }
                            Err(err) => {
                                error!("{}", err);
                            }
                        }
                    }
                    Err(err) => {
                        error!("{}", err);
                    }
                }
            }
        }

        Ok(())
    }

    pub fn delete_volume(&mut self, vid: VolumeId) -> Result<()> {
        panic!("TODO");

        Ok(())
    }
}
