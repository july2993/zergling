
use std::collections::HashMap;
use std::collections::HashSet;

use rand;
use std::sync::Arc;
use std::cell::RefCell;

use storage::{VolumeId, ReplicaPlacement, TTL};
use directory::topology::{DataNode, VolumeGrowOption, VolumeInfo};

use directory::errors::{Result,Error};
use storage;


#[derive(Debug)]
pub struct VolumeLayout {
    pub rp: ReplicaPlacement,
    pub ttl: Option<TTL>,
    pub volume_size_limit: u64,

    pub writable_volumes: Vec<VolumeId>,
    pub readonly_volumes: HashSet<VolumeId>,
    pub oversize_volumes: HashSet<VolumeId>,

    pub vid2location: HashMap<VolumeId, Vec<Arc<RefCell<DataNode>>>>
}

impl VolumeLayout {
    pub fn new(rp: ReplicaPlacement, ttl: Option<TTL>, volume_size_limit: u64) -> VolumeLayout {
        VolumeLayout {
            rp: rp,
            ttl: ttl,
            volume_size_limit: volume_size_limit,
            writable_volumes: Vec::new(),
            readonly_volumes: HashSet::new(),
            oversize_volumes: HashSet::new(),
            vid2location: HashMap::new(),
        }
    }

    // get match data_center, rack, node volume count
    pub fn get_active_volume_count(&self, option: &VolumeGrowOption) -> i64 {
        if option.data_center == "" {
            return self.writable_volumes.len() as i64;
        }
        let mut count = 0;

        for vid in &self.writable_volumes {
            for node in self.vid2location.get(vid).unwrap_or(&vec![]) {
                let bnode = node.borrow();
                if bnode.id == option.data_node 
                && bnode.get_rack_id() == option.rack 
                && bnode.get_data_center_id() == option.data_center {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn pick_for_write(&self, option: &VolumeGrowOption) -> Result<(VolumeId, Vec<Arc<RefCell<DataNode>>>)> {
        if self.writable_volumes.len() <= 0 {
            return Err(Error::NoWritableVolume(String::from("no writable volumes")));
        }

        let mut counter = 0;
        let mut ret = (0, vec![]);

        for vid in &self.writable_volumes {
            match self.vid2location.get(&vid) {
                None => (),
                Some(location) => {
                    for node in location {
                        let dn = node.borrow();
                        if option.data_center != "" && option.data_center != dn.get_data_center_id()
                            || option.rack != "" && option.rack != dn.get_rack_id()
                            || option.data_node != "" && option.data_node != dn.id {
                                continue
                            }
                        
                        counter += 1;
                        if rand::random::<i64>() % counter < 1 {
                            let mut lo = vec![];
                            for n in location {
                                lo.push(n.clone());
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

    fn set_node(list: &mut Vec<Arc<RefCell<DataNode>>>, nd: Arc<RefCell<DataNode>>) {
        // debug!("set node: {:?} {:?}", list, nd);
        let mut same: Option<usize> = None;
        let mut i = 0;
        for e in list.iter() {
            let e_ref = e.borrow();
            let nd_ref = nd.borrow();
            if e_ref.ip != nd_ref.ip || e_ref.port != nd_ref.port {
                i += 1;
                continue
            }

            same = Some(i);
            break;
        }
        if let Some(idx) = same {
            list[idx] = nd.clone();
        } else {
            list.push(nd.clone())
        }
    }

    pub fn register_volume(&mut self, v: &VolumeInfo, dn: Arc<RefCell<DataNode>>) {
        {
            let list = self.vid2location.entry(v.id) 
                .or_insert(vec![]);
            VolumeLayout::set_node(list, dn.clone());
        }

       let list = self.vid2location.get(&v.id).unwrap().clone();

       for node in list.iter() {
           match node.borrow().volumes.get(&v.id) {
               Some(v) => {
                    if v.read_only {
                        self.remove_from_writable(v.id);
                        self.readonly_volumes.insert(v.id);
                    }
               },
               None => {
                   self.remove_from_writable(v.id);
                   self.readonly_volumes.remove(&v.id);
               }
           }
       }

       if list.len() == self.rp.get_copy_count() as usize && self.is_writable(v) {
            if self.oversize_volumes.get(&v.id).is_none() {
                self.add_to_writable(v.id);
            }
       } else {
            self.remove_from_writable(v.id);
            self.set_oversized_if_need(v);
       }

    }

    fn set_oversized_if_need(&mut self, v: &VolumeInfo) {
        if self.is_oversized(v) {
            self.oversize_volumes.insert(v.id);
        }
    }

    fn is_oversized(&self, v: &VolumeInfo) -> bool {
        return v.size >= self.volume_size_limit
    }

    fn is_writable(&self, v: &VolumeInfo) -> bool {
        return !self.is_oversized(v) &&
            v.version == storage::CURRENT_VERSION &&
            !v.read_only
    }

    fn add_to_writable(&mut self, vid: VolumeId) {
        for id in self.writable_volumes.iter() {
            if *id == vid {
                return;
            }
        }
        self.writable_volumes.push(vid);
    }

    fn remove_from_writable(&mut self, vid: VolumeId) {
        let mut idx: Option<usize> = None;
        let mut i = 0;
        for v in self.writable_volumes.iter() {
            if *v == vid {
                idx = Some(i);
                break;
            }

            i = i + 1;
        }

        if idx.is_some() {
            self.writable_volumes.remove(idx.unwrap());
        }
    }

    pub fn un_register_volume(&mut self, v: &VolumeInfo, _dn: Arc<RefCell<DataNode>>) {
        self.remove_from_writable(v.id);
    }

    pub fn lookup(&self, vid: VolumeId) -> Option<Vec<Arc<RefCell<DataNode>>>> {
        match self.vid2location.get(&vid) {
            Some(list) => Some(list.clone()),
            None => None,
        }
    }
}

