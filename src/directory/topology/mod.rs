mod topology;
mod collection;
mod volume_layout;
mod volume_grow;

use std::collections::HashMap;
use std::sync::Arc;
use std::cell::RefCell;
use std::sync::Weak;
use directory::errors::{Result, Error};

use rand::random;

pub use self::topology::Topology;
pub use self::collection::Collection;
pub use self::volume_layout::VolumeLayout;
pub use self::volume_grow::{VolumeGrow, VolumeGrowOption};

pub use storage::{ReplicaPlacement,TTL, VolumeId, VolumeInfo};


#[derive(Debug,Default,Clone)]
pub struct DataNode {
    pub id: String,
    pub ip: String,
    pub port: i64,
    pub public_url: String,
    pub last_seen: i64,
    pub rack: Weak<RefCell<Rack>>,
    pub volumes: HashMap<VolumeId, VolumeInfo>,
    pub max_volumes: i64,
    pub max_volume_id: VolumeId,
}

unsafe impl Send for DataNode {}

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
        match  self.rack.upgrade() {
            Some(rack) => return rack.borrow().get_data_center_id(),
            None => String::from(""),
        }
    }

    pub fn update_volumes(&mut self, infos: Vec<VolumeInfo>) -> Vec<VolumeInfo> {
        debug!("{} - update_volumes {:?}", self.id, infos);
        let mut infos_map = HashMap::new();
        for info in infos.iter() {
            infos_map.insert(info.id, info.clone());
        }

        let mut deleted_id: Vec<VolumeId> = vec![];
        let mut deleted: Vec<VolumeInfo> = vec![];

        for (id, has) in self.volumes.iter_mut() {
            match infos_map.get(&id) {
                Some(new) => *has = new.clone(),
                None => deleted_id.push(has.id),
            }
        }

        for id in deleted_id.iter() {
            deleted.push(self.volumes.remove(id).unwrap())
        }


        deleted
    }
}

#[derive(Debug)]
pub struct Rack {
    pub id: String,
    pub nodes: HashMap<String, Arc<RefCell<DataNode>>>,
    pub max_volume_id: VolumeId,

    pub data_center: Weak<RefCell<DataCenter>>,
	
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

	pub fn get_or_create_data_node(&mut self, id: &str, ip: &str, port: i64, public_url: &str, max_volumes: i64) -> Arc<RefCell<DataNode>> {
		let node = self.nodes
			.entry(String::from(id))
			.or_insert(Arc::new(RefCell::new(DataNode::new(id, ip, port, public_url, max_volumes,))));
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
                return Ok(node.clone())
            }
        }

        return  Err(Error::NoFreeSpace);
    }
}


#[derive(Debug)]
pub struct DataCenter {
    pub id: String,
    pub max_volume_id: VolumeId,
    pub racks: HashMap<String, Arc<RefCell<Rack>>>,
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

        Err(Error::NoFreeSpace)
    }
}


