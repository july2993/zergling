

mod topology;
mod collection;
mod volume_layout;
mod volume_grow;

use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;



pub use self::topology::Topology;
pub use self::collection::Collection;
pub use self::volume_layout::VolumeLayout;
pub use self::volume_grow::{VolumeGrow, VolumeGrowOption};

pub use storage::{ReplicaPlacement,TTL, VolumeId, VolumeInfo};


#[derive(Debug,Default)]
pub struct DataNode {
    pub id: String,
    pub ip: String,
    pub port: i64,
    pub public_url: String,
    pub last_seen: i64,
    pub rack: Arc<Option<Rack>>,
    pub volumes: HashMap<VolumeId, VolumeInfo>,
    pub max_volumes: i64,
}

impl DataNode {
    pub fn url(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }

    pub fn add_or_update_volume(&mut self, v: VolumeInfo) {
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
        // too weird...
        match self.rack.as_ref() {
            &Some(ref rack) => return rack.id.clone(),
            &None => String::from(""),
        }
    }

    pub fn get_data_center_id(&self) -> String {
        // too weird...
        match self.rack.as_ref() {
            &Some(ref rack) => return rack.get_data_center_id(),
            &None => String::from(""),
        }
    }
}

#[derive(Debug)]
pub struct Rack {
    pub id: String,
    pub nodes: HashMap<String, DataNode>,

    pub data_center: Arc<Option<DataCenter>>,
}

impl Rack {
    fn new(id: &str) -> Rack {
        Rack {
            id: String::from(id),
            nodes: HashMap::new(),
            data_center: Arc::new(None),
        }
    }

    pub fn get_data_center_id(&self) -> String {
        // too weird...
        match self.data_center.as_ref() {
            &Some(ref data_center) => return data_center.id.clone(),
            &None => String::from(""),
        }
    }

    pub fn has_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, nd) in &self.nodes {
            ret += nd.has_volumes();
        }
        ret
    }

    pub fn max_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, nd) in &self.nodes {
            ret += nd.max_volumes();
        }
        ret
    }

    pub fn free_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, nd) in &self.nodes {
            ret += nd.free_volumes();
        }
        ret
    }
}


#[derive(Debug)]
pub struct DataCenter {
    pub id: String,
    pub racks: HashMap<String, Rack>,
}

impl DataCenter {
    fn new(id: &str) -> DataCenter {
        DataCenter {
            id: String::from(id),
            racks: HashMap::new(),
        }
    }

    pub fn has_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, rack) in &self.racks {
            ret += rack.has_volumes();
        }
        ret
    }

    pub fn max_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, rack) in &self.racks {
            ret += rack.max_volumes();
        }
        ret
    }

    pub fn free_volumes(&self) -> i64 {
        let mut ret = 0;
        for (id, rack) in &self.racks {
            ret += rack.free_volumes();
        }
        ret
    }
}


