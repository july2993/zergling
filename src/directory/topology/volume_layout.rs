
use std::collections::HashMap;
use std::collections::HashSet;

use rand;

use storage::{VolumeId, ReplicaPlacement, TTL};
use directory::topology::{DataNode, VolumeGrowOption, VolumeInfo};

use directory::errors::{Result,Error};


#[derive(Debug)]
pub struct VolumeLayout {
    pub rp: ReplicaPlacement,
    pub ttl: Option<TTL>,
    pub volumeSizeLimit: u64,

    pub writableVolumes: Vec<VolumeId>,
    pub readonlyVolumes: HashSet<VolumeId>,
    pub oversizeVolumes: HashSet<VolumeId>,

    pub vid2location: HashMap<VolumeId, Vec<DataNode>>
}

impl VolumeLayout {
    pub fn new(rp: ReplicaPlacement, ttl: Option<TTL>, volumeSizeLimit: u64) -> VolumeLayout {
        VolumeLayout {
            rp: rp,
            ttl: ttl,
            volumeSizeLimit: volumeSizeLimit,
            writableVolumes: Vec::new(),
            readonlyVolumes: HashSet::new(),
            oversizeVolumes: HashSet::new(),
            vid2location: HashMap::new(),
        }
    }

    // get match data_center, rack, node volume count
    pub fn get_active_volume_count(&self, option: &VolumeGrowOption) -> i64 {
        if option.DataCenter == "" {
            return self.writableVolumes.len() as i64;
        }
        let mut count = 0;

        for vid in &self.writableVolumes {
            for node in self.vid2location.get(vid).unwrap_or(&vec![]) {
                if node.id == option.DataNode 
                && node.get_rack_id() == option.Rack 
                && node.get_data_center_id() == option.DataCenter {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn pick_for_write(&self, count: u64, option: &VolumeGrowOption) -> Result<(VolumeId, Vec<&DataNode>)> {
        if self.writableVolumes.len() < 0 {
            return Err(Error::NoWritableVolume(String::from("no writable volumes")));
        }

        let mut counter = 0;
        let mut ret = (0, vec![]);

        for vid in &self.writableVolumes {
            match self.vid2location.get(&vid) {
                None => (),
                Some(location) => {
                    for dn in location {
                        if option.DataCenter != "" && option.DataCenter != dn.get_data_center_id()
                            || option.Rack != "" && option.Rack != dn.get_rack_id()
                            || option.DataNode != "" && option.DataNode != dn.id {
                                continue
                            }
                        
                        counter += 1;
                        if rand::random::<i64>() % counter < 1 {
                            let mut lo = vec![];
                            for n in location {
                                lo.push(n);
                            }
                            ret = (*vid, lo);
                        }
                    }
                }
            }
        }

        if counter > 0 {
            return Ok(ret);
        }

        return Err(Error::NoWritableVolume(String::from("no match node")));
    }

    pub fn register_volume(&mut self, v: VolumeInfo, dn: DataNode) {

    }
}

