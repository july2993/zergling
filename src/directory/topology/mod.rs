mod topology;
mod collection;
mod volume_layout;
mod volume_grow;

use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Weak;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use directory::errors::{Error, Result};
use std::fmt;

use rand::random;

pub use self::topology::Topology;
pub use self::collection::Collection;
pub use self::volume_layout::VolumeLayout;
pub use self::volume_grow::{VolumeGrow, VolumeGrowOption};

pub use storage::{ReplicaPlacement, VolumeId, VolumeInfo, TTL};



#[derive(Debug, Default, Clone, Serialize)]
pub struct DataNode {
    pub id: String,
    pub ip: String,
    pub port: i64,
    pub public_url: String,
    pub last_seen: i64,
    #[serde(skip)] pub rack: Weak<RefCell<Rack>>,
    pub volumes: HashMap<VolumeId, VolumeInfo>,
    pub max_volumes: i64,
    pub max_volume_id: VolumeId,
}

// TODO
unsafe impl Send for DataNode {}

impl fmt::Display for DataNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, volumes: {})", self.id, self.volumes.len())
    }
}


impl DataNode {
    pub fn new(id: &str, ip: &str, port: i64, public_url: &str, max_volumes: i64) -> DataNode {
        DataNode {
            id: String::from(id),
            ip: String::from(ip),
            port: port,
            public_url: String::from(public_url),
            last_seen: 0,
            rack: Weak::default(),
            volumes: HashMap::new(),
            max_volumes: max_volumes,
            max_volume_id: 0,
        }
    }

    pub fn url(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn adjust_max_volume_id(&mut self, vid: VolumeId) {
        if vid > self.max_volume_id {
            self.max_volume_id = vid;
        }

        if let Some(rack) = self.rack.upgrade() {
            rack.borrow_mut().adjust_max_volume_id(self.max_volume_id);
        }
    }

    pub fn add_or_update_volume(&mut self, v: VolumeInfo) {
        self.adjust_max_volume_id(v.id);
        self.volumes.insert(v.id, v);
    }

    pub fn has_volumes(&self) -> i64 {
        self.volumes.len() as i64
    }

    pub fn max_volumes(&self) -> i64 {
        self.max_volumes
    }

    pub fn free_volumes(&self) -> i64 {
        self.max_volumes() - self.has_volumes()
    }

    pub fn get_rack_id(&self) -> String {
        match self.rack.upgrade() {
            Some(rack) => return rack.borrow().id.clone(),
            None => String::from(""),
        }
    }

    pub fn get_data_center_id(&self) -> String {
        match self.rack.upgrade() {
            Some(rack) => return rack.borrow().get_data_center_id(),
            None => String::from(""),
        }
    }

    pub fn update_volumes(&mut self, infos: Vec<VolumeInfo>) -> Vec<VolumeInfo> {
        // debug!("{} - update_volumes {:?}", self.id, infos);
        let mut infos_map = HashMap::new();
        for info in infos.iter() {
            infos_map.insert(info.id, info.clone());
        }

        let mut deleted_id: Vec<VolumeId> = vec![];
        let mut deleted: Vec<VolumeInfo> = vec![];

        for (id, has) in self.volumes.iter_mut() {
            match infos_map.get(&id) {
                Some(_) => {}
                None => deleted_id.push(has.id),
            }
        }

        for vi in infos.iter() {
            self.add_or_update_volume(vi.clone());
        }

        for id in deleted_id.iter() {
            deleted.push(self.volumes.remove(id).unwrap())
        }

        deleted
    }
}

#[derive(Clone, Debug)]
pub struct Rack {
    pub id: String,
    // #[serde(skip)]
    pub nodes: HashMap<String, Arc<RefCell<DataNode>>>,
    pub max_volume_id: VolumeId,
    // #[serde(skip)]
    pub data_center: Weak<RefCell<DataCenter>>,
}

impl Serialize for Rack {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Rack", 3)?;
        state.serialize_field("id", &self.id)?;
        let mut nodes = vec![];
        for (_, n) in self.nodes.iter() {
            nodes.push(n.borrow().clone());
        }
        state.serialize_field("nodes", &nodes)?;
        state.serialize_field("max_volume_id", &self.max_volume_id)?;
        state.end()
    }
}


impl Rack {
    fn new(id: &str) -> Rack {
        Rack {
            id: String::from(id),
            nodes: HashMap::new(),
            data_center: Weak::new(),
            max_volume_id: 0,
        }
    }

    pub fn adjust_max_volume_id(&mut self, vid: VolumeId) {
        if vid > self.max_volume_id {
            self.max_volume_id = vid;
        }

        if let Some(dc) = self.data_center.upgrade() {
            dc.borrow_mut().adjust_max_volume_id(self.max_volume_id);
        }
    }

    pub fn get_or_create_data_node(
        &mut self,
        id: &str,
        ip: &str,
        port: i64,
        public_url: &str,
        max_volumes: i64,
    ) -> Arc<RefCell<DataNode>> {
        let node = self.nodes
            .entry(String::from(id))
            .or_insert(Arc::new(RefCell::new(
                DataNode::new(id, ip, port, public_url, max_volumes),
            )));
        node.clone()
    }

    pub fn get_data_center_id(&self) -> String {
        match self.data_center.upgrade() {
            Some(data_center) => return data_center.borrow().id.clone(),
            None => String::from(""),
        }
    }

    pub fn has_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, nd) in &self.nodes {
            ret += nd.borrow().has_volumes();
        }
        ret
    }

    pub fn max_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, nd) in &self.nodes {
            ret += nd.borrow().max_volumes();
        }
        ret
    }

    pub fn free_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, nd) in &self.nodes {
            ret += nd.borrow().free_volumes();
        }
        ret
    }

    pub fn reserve_one_volume(&self) -> Result<Arc<RefCell<DataNode>>> {
        // randomly select
        let mut free_volumes = 0;
        for (_, node) in self.nodes.iter() {
            free_volumes += node.borrow().free_volumes();
        }

        let idx = random::<u32>() as i64 % free_volumes;

        for (_, node) in self.nodes.iter() {
            free_volumes -= node.borrow().free_volumes();
            if free_volumes == idx {
                return Ok(node.clone());
            }
        }

        return Err(Error::NoFreeSpace(
            format!("reserve_one_volume on rack {} fail", self.id),
        ));
    }
}


#[derive(Debug, Clone)]
pub struct DataCenter {
    pub id: String,
    pub max_volume_id: VolumeId,
    pub racks: HashMap<String, Arc<RefCell<Rack>>>,
}

impl Serialize for DataCenter {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DataCenter", 3)?;
        state.serialize_field("id", &self.id)?;
        let mut racks = vec![];
        for (_, r) in self.racks.iter() {
            racks.push(r.borrow().clone());
        }
        state.serialize_field("racks", &racks)?;
        state.serialize_field("max_volume_id", &self.max_volume_id)?;
        state.end()
    }
}

impl DataCenter {
    fn new(id: &str) -> DataCenter {
        DataCenter {
            id: String::from(id),
            racks: HashMap::new(),
            max_volume_id: 0,
        }
    }

    pub fn adjust_max_volume_id(&mut self, vid: VolumeId) {
        if vid > self.max_volume_id {
            self.max_volume_id = vid;
        }
    }

    pub fn get_or_create_rack(&mut self, id: &str) -> Arc<RefCell<Rack>> {
        self.racks
            .entry(String::from(id))
            .or_insert(Arc::new(RefCell::new(Rack::new(id))))
            .clone()
    }

    pub fn has_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, rack) in &self.racks {
            ret += rack.borrow().has_volumes();
        }
        ret
    }

    pub fn max_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, rack) in &self.racks {
            ret += rack.borrow().max_volumes();
        }
        ret
    }

    pub fn free_volumes(&self) -> i64 {
        let mut ret = 0;
        for (_id, rack) in &self.racks {
            ret += rack.borrow().free_volumes();
        }
        ret
    }

    pub fn reserve_one_volume(&self) -> Result<Arc<RefCell<DataNode>>> {
        // randomly select one
        let mut free_volumes = 0;
        for (_, rack) in self.racks.iter() {
            free_volumes += rack.borrow().free_volumes();
        }

        let idx = random::<u32>() as i64 % free_volumes;

        for (_, rack) in self.racks.iter() {
            free_volumes -= rack.borrow().free_volumes();
            if free_volumes == idx {
                return rack.borrow().reserve_one_volume();
            }
        }

        Err(Error::NoFreeSpace(String::from(
            format!("reserve_one_volume on dc {} fail", self.id),
        )))
    }
}
