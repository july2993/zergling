
use storage;
use storage::needle;
use storage::{DiskLocation, VolumeId, Needle, Result, Volume, Error, NeedleMapType,
              ReplicaPlacement, TTL};
use pb;

const MAX_TTL_VOLUME_REMOVAL_DELAY_MINUTES: u64 = 10;

// #[derive(Default)]
// #[derive(Serialize, Deserialize)]
pub struct Store {
    pub ip: String,
    pub port: u16,
    pub public_url: String,
    pub locations: Vec<storage::DiskLocation>,

    pub data_center: String,
    pub rack: String,
    pub connected: bool,
    pub volume_size_limit: u64, // read from master
    pub client: Option<pb::zergling_grpc::SeaweedClient>,
    pub needle_map_type: storage::NeedleMapType,
}


impl Store {
    pub fn new(
        ip: &str,
        port: u16,
        public_url: &str,
        folders: Vec<String>,
        max_counts: Vec<i64>,
        needle_map_kind: storage::NeedleMapType,
    ) -> Store {

        let mut locations = vec![];
        for i in 0..folders.len() {
            let mut location = DiskLocation::new(&folders[i], max_counts[i]);
            location.load_existing_volumes(needle_map_kind).unwrap();
            locations.push(location);
        }

        let s = Store {
            ip: String::from(ip),
            port: port,
            public_url: String::from(public_url),
            locations: locations,
            client: None,
            needle_map_type: needle_map_kind,
            data_center: String::from(""),
            rack: String::from(""),
            connected: false,
            volume_size_limit: 0,
        };

        s
    }

    pub fn find_volume_mut(&mut self, vid: VolumeId) -> Option<&mut Volume> {
        for location in self.locations.iter_mut() {
            let ret = location.volumes.get_mut(&vid);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }

    pub fn has_volume(&self, vid: VolumeId) -> bool {
        self.find_volume(vid).is_some()
    }

    pub fn find_volume(&self, vid: VolumeId) -> Option<&Volume> {
        for location in self.locations.iter() {
            let ret = location.volumes.get(&vid);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }

    pub fn delete_volume_needle(&mut self, vid: VolumeId, n: &Needle) -> Result<u32> {
        if let Some(v) = self.find_volume_mut(vid) {
            return v.delete_needle(n);
        }

        Ok(0)
    }

    pub fn read_volume_needle(&mut self, vid: VolumeId, n: &mut Needle) -> Result<u32> {
        if let Some(v) = self.find_volume_mut(vid) {
            return v.read_needle(n);
        }
        return Err(box_err!("volume {} not found", vid));
    }

    pub fn write_volume_needle(&mut self, vid: VolumeId, n: &Needle) -> Result<u32> {
        if let Some(v) = self.find_volume_mut(vid) {
            if v.read_only {
                return Err(box_err!("volume {} is read only", vid));
            }

            // TODO what
            // if v.content_size() > needle::MAX_POSSIBLE_VOLUME_SIZE {
            if false {
                return Err(box_err!("volume {} is read only", vid));
            }

            return v.write_needle(n);

        } else {
            return Err((box_err!("volume {} not fount", vid)));
        }
    }

    pub fn delete_volume(&mut self, vid: VolumeId) -> Result<()> {
        let mut delete = false;
        for location in self.locations.iter_mut() {
            if location.delete_volume(vid).is_ok() {
                delete = true;
            }
        }
        if delete {
            self.update_master();
            return Ok(());
        } else {
            return Err((box_err!("volume {} not fount on dis", vid)));
        }
    }

    pub fn update_master(&self) {
        panic!("TODO");
    }

    fn find_free_location(&mut self) -> Option<&mut DiskLocation> {
        let mut rt = None;
        let mut max_free: i64 = 0;
        for location in self.locations.iter_mut() {
            let free = location.max_volume_count - location.volumes.len() as i64;
            if free > max_free {
                max_free = free;
                rt = Some(location);
            }
        }

        rt
    }

    fn do_add_volume(
        &mut self,
        vid: VolumeId,
        collection: &str,
        needle_map_kind: NeedleMapType,
        replica_placement: ReplicaPlacement,
        ttl: TTL,
        pre_allocate: i64,
    ) -> Result<()> {
        if self.find_volume(vid).is_some() {
            return Err(box_err!("volume id {} already exists!", vid));
        }

        let location = self.find_free_location().ok_or::<Error>(
            box_err!("no more free space left"),
        )?;
        let volume = Volume::new(
            &location.directory,
            collection,
            vid,
            needle_map_kind,
            replica_placement,
            ttl,
            pre_allocate,
        )?;

        location.volumes.insert(vid, volume);

        Ok(())
    }



    pub fn add_volume(
        &mut self,
        volume_list: &str,
        collection: &str,
        needle_map_kind: NeedleMapType,
        replica_placement: &str,
        ttl_str: &str,
        pre_allocate: i64,
    ) -> Result<()> {

        let rp = ReplicaPlacement::new(replica_placement)?;
        let ttl = TTL::new(ttl_str)?;

        for range_str in volume_list.split(",") {
            let parts: Vec<&str> = range_str.split("-").collect();
            if parts.len() == 1 {
                let id_str = parts[0];
                let vid = id_str.parse::<u32>()?;
                self.do_add_volume(
                    vid,
                    collection,
                    needle_map_kind,
                    rp,
                    ttl,
                    pre_allocate,
                )?;
            } else {
                let start = parts[0].parse::<u32>()?;
                let end = parts[1].parse::<u32>()?;

                for id in start..(end + 1) {
                    self.do_add_volume(
                        id,
                        collection,
                        needle_map_kind,
                        rp,
                        ttl,
                        pre_allocate,
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn collect_heartbeat(&mut self) -> pb::zergling::Heartbeat {
        let mut beat = pb::zergling::Heartbeat::default();

        let mut max_file_key: u64 = 0;
        let mut max_volume_count = 0;
        for location in self.locations.iter_mut() {
            let mut deleted_vids = Vec::new();
            max_volume_count += location.max_volume_count;
            for (vid, v) in location.volumes.iter() {
                if v.nm.max_file_key() > max_file_key {
                    max_file_key = v.nm.max_file_key();
                }

                if v.expired(self.volume_size_limit) {
                    let mut msg = pb::zergling::VolumeInformationMessage::new();
                    msg.set_id(*vid);
                    msg.set_size(v.size().unwrap_or(0));
                    msg.set_collection(v.collection.clone());
                    msg.set_file_count(v.nm.file_count());
                    msg.set_delete_count(v.nm.delete_count());
                    msg.set_deleted_byte_count(v.nm.deleted_byte_count());
                    msg.set_read_only(v.read_only);
                    msg.set_replica_placement(v.super_block.replica_placement.byte() as u32);
                    msg.set_version(v.super_block.version as u32);
                    msg.set_ttl(v.super_block.ttl.to_u32());
                    beat.volumes.push(msg);
                } else {
                    if v.expired_long_enough(MAX_TTL_VOLUME_REMOVAL_DELAY_MINUTES) {
                        deleted_vids.push(v.id);
                        info!("volume {} is deleted.", v.id);
                    } else {
                        info!("volume {} is expired.", *vid);
                    }
                }
            }
            for vid in deleted_vids {
                if let Err(err) = location.delete_volume(vid) {
                    warn!("delete volume {} err: {}", vid, err);
                }
            }
        }

        beat.ip = self.ip.clone();
        beat.port = self.port as u32;
        beat.public_url = self.public_url.clone();
        beat.max_volume_count = max_volume_count as u32;
        beat.max_file_key = max_file_key;
        beat.data_center = self.data_center.clone();
        beat.rack = self.rack.clone();

        beat
    }
}
