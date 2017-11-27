use directory::topology::{DataCenter, DataNode, Rack, Topology, VolumeInfo};
use directory::{Error, Result};
use storage;
use storage::VolumeId;
use std::cell::RefCell;
use std::sync::Arc;
use rand::{random, thread_rng, Rng};


use util;

use serde_json;
use serde_json::Value;


#[derive(Debug)]
pub struct VolumeGrow {}

impl VolumeGrow {
    pub fn new() -> VolumeGrow {
        VolumeGrow {}
    }

    pub fn grow_by_type(&self, option: &VolumeGrowOption, topo: &mut Topology) -> Result<i64> {
        let count = self.logical_count(option.replica_placement.get_copy_count());
        self.grow_by_count_and_type(count, option, topo)
    }

    fn logical_count(&self, pcount: i64) -> i64 {
        match pcount {
            1 => 7,
            2 => 6,
            3 => 3,
            _ => 1,
        }
    }

    // TODO: too long func...
    // will specify data_node but no data center find a wrong data center to be the
    // main data center first, then no valid data_node ???
    fn find_empty_slots(
        &self,
        option: &VolumeGrowOption,
        topo: &Topology,
    ) -> Result<Vec<Arc<RefCell<DataNode>>>> {
        let mut main_dc: Option<Arc<RefCell<DataCenter>>> = None;
        let mut main_rack: Option<Arc<RefCell<Rack>>> = None;
        let mut main_nd: Option<Arc<RefCell<DataNode>>> = None;
        let mut other_centers: Vec<Arc<RefCell<DataCenter>>> = vec![];
        let mut other_racks: Vec<Arc<RefCell<Rack>>> = vec![];
        let mut other_nodes: Vec<Arc<RefCell<DataNode>>> = vec![];

        let rp = option.replica_placement;
        let mut valid_main_counts = 0;
        // find main data center
        for (_dc_id, dc_arc) in topo.data_centers.iter() {
            let dc = dc_arc.borrow();
            if option.data_center != "" && dc.id != option.data_center {
                continue;
            }

            if dc.racks.len() < rp.diff_rack_count as usize + 1 {
                continue;
            }

            if dc.free_volumes() < rp.diff_rack_count as i64 + rp.same_rack_count as i64 + 1 {
                continue;
            }

            let mut possible_racks_count = 0;
            for (_rack_id, rack_arc) in dc.racks.iter() {
                let rack = rack_arc.borrow();
                let mut possible_nodes_count = 0;
                for (_, nd) in rack.nodes.iter() {
                    if nd.borrow().free_volumes() >= 1 {
                        possible_nodes_count += 1;
                    }
                }

                if possible_nodes_count >= rp.same_rack_count + 1 {
                    possible_racks_count += 1;
                }
            }

            if possible_racks_count < rp.diff_rack_count + 1 {
                continue;
            }

            valid_main_counts += 1;
            if random::<u32>() % valid_main_counts == 0 {
                main_dc = Some(dc_arc.clone());
            }
        }

        if main_dc.is_none() {
            return Err(Error::NoFreeSpace("find main dc fail".to_string()));
        }
        let main_dc_arc = main_dc.unwrap();


        if rp.diff_data_center_count > 0 {
            for (dc_id, dc_arc) in topo.data_centers.iter() {
                let dc = dc_arc.borrow();
                if *dc_id == main_dc_arc.borrow().id || dc.free_volumes() < 1 {
                    continue;
                }
                other_centers.push(dc_arc.clone());
            }
        }
        if other_centers.len() < rp.diff_data_center_count as usize {
            return Err(Error::NoFreeSpace("no enough data centers".to_string()));
        }

        thread_rng().shuffle(other_centers.as_mut_slice());
        let tmp_centers = other_centers
            .drain(0..rp.diff_data_center_count as usize)
            .collect();
        other_centers = tmp_centers;


        // find main rack
        let mut valid_rack_count = 0;
        for (_rack_id, rack_arc) in main_dc_arc.borrow().racks.iter() {
            let rack = rack_arc.borrow();
            if option.rack != "" && option.rack != rack.id {
                continue;
            }

            if rack.free_volumes() < rp.same_rack_count as i64 + 1 {
                continue;
            }

            if rack.nodes.len() < rp.same_rack_count as usize + 1 {
                continue;
            }

            let mut possible_nodes = 0;
            for (_node_id, node) in rack.nodes.iter() {
                if node.borrow().free_volumes() < 1 {
                    continue;
                }

                possible_nodes += 1;
            }

            if possible_nodes < rp.same_rack_count as usize + 1 {
                continue;
            }
            valid_rack_count += 1;

            if random::<u32>() % valid_rack_count == 0 {
                main_rack = Some(rack_arc.clone());
            }
        }

        if main_rack.is_none() {
            return Err(Error::NoFreeSpace("find main rack fail".to_string()));
        }

        let main_rack_arc = main_rack.unwrap();

        if rp.diff_rack_count > 0 {
            for (rack_id, rack_arc) in main_dc_arc.borrow().racks.iter() {
                let rack = rack_arc.borrow();
                if *rack_id == main_rack_arc.borrow().id || rack.free_volumes() < 1 {
                    continue;
                }
                other_racks.push(rack_arc.clone());
            }
        }

        if other_racks.len() < rp.diff_rack_count as usize {
            return Err(Error::NoFreeSpace("no enough racks".to_string()));
        }

        thread_rng().shuffle(other_racks.as_mut_slice());
        let tmp_racks = other_racks.drain(0..rp.diff_rack_count as usize).collect();
        other_racks = tmp_racks;


        // find main node
        let mut valid_node = 0;
        for (node_id, node) in main_rack_arc.borrow().nodes.iter() {
            if option.data_node != "" && option.data_node != *node_id {
                continue;
            }
            if node.borrow().free_volumes() < 1 {
                continue;
            }

            valid_node += 1;
            if random::<u32>() % valid_node == 0 {
                main_nd = Some(node.clone());
            }
        }

        if main_nd.is_none() {
            return Err(Error::NoFreeSpace("find main node fail".to_string()));
        }
        let main_nd_arc = main_nd.unwrap().clone();


        if rp.same_rack_count > 0 {
            for (node_id, node) in main_rack_arc.borrow().nodes.iter() {
                let node_ref = node.borrow();

                if *node_id == main_nd_arc.borrow().id || node_ref.free_volumes() < 1 {
                    continue;
                }
                other_nodes.push(node.clone());
            }
        }

        if other_nodes.len() < rp.same_rack_count as usize {
            return Err(Error::NoFreeSpace("no enough  nodes".to_string()));
        }
        thread_rng().shuffle(other_nodes.as_mut_slice());
        let tmp_nodes = other_nodes.drain(0..rp.same_rack_count as usize).collect();
        other_nodes = tmp_nodes;


        let mut ret = vec![];
        ret.push(main_nd_arc.clone());

        for nd in other_nodes {
            ret.push(nd.clone());
        }

        for rack in other_racks {
            let node = rack.borrow().reserve_one_volume()?;
            ret.push(node);
        }

        for dc in other_centers {
            let node = dc.borrow().reserve_one_volume()?;
            ret.push(node);
        }


        Ok(ret)
    }

    fn find_and_grow(&self, option: &VolumeGrowOption, topo: &mut Topology) -> Result<i64> {
        let nodes = self.find_empty_slots(option, topo)?;
        let len = nodes.len();
        let vid = topo.next_volume_id();
        self.grow(vid, option, topo, nodes)?;
        Ok(len as i64)
    }

    fn grow_by_count_and_type(
        &self,
        count: i64,
        option: &VolumeGrowOption,
        topo: &mut Topology,
    ) -> Result<i64> {
        let mut grow_count = 0;
        for _ in 0..count {
            grow_count += self.find_and_grow(option, topo)?;
        }

        Ok(grow_count)
    }

    fn grow(
        &self,
        vid: VolumeId,
        option: &VolumeGrowOption,
        topo: &mut Topology,
        nodes: Vec<Arc<RefCell<DataNode>>>,
    ) -> Result<()> {
        for nd in nodes {
            allocate_volume(&nd.borrow(), vid, option)?;
            let volume_info = VolumeInfo {
                id: vid,
                size: 0,
                collection: option.collection.clone(),
                replica_placement: option.replica_placement,
                ttl: option.ttl,
                version: storage::CURRENT_VERSION,
                ..Default::default()
            };

            {
                let mut mut_node = nd.borrow_mut();
                mut_node.add_or_update_volume(volume_info.clone());
            }

            topo.register_volume_layout(volume_info, nd.clone());
        }
        Ok(())
    }
}


#[derive(Debug, Default)]
pub struct VolumeGrowOption {
    pub collection: String,
    pub replica_placement: storage::ReplicaPlacement,
    pub ttl: storage::TTL,
    pub preallocate: i64,
    pub data_center: String,
    pub rack: String,
    pub data_node: String,
}

impl VolumeGrowOption {}


fn allocate_volume(dn: &DataNode, vid: VolumeId, option: &VolumeGrowOption) -> Result<()> {
    let vid_p = &vid.to_string();
    let collection_p = &option.collection;
    let rp_p = &option.replica_placement.string();
    let ttl_p = &option.ttl.string();
    let pre_p = &format!("{}", option.preallocate);

    let mut params: Vec<(&str, &str)> = vec![];

    params.push(("volume", vid_p));
    params.push(("collection", collection_p));
    params.push(("replication", rp_p));
    params.push(("ttl", ttl_p));
    params.push(("preallocate", pre_p));

    let body = util::post(&format!("http://{}/admin/assign_volume", dn.url()), &params)?;

    let v: serde_json::Value = serde_json::from_slice(&body)?;

    if let Value::String(ref s) = v["Error"] {
        return Err(Error::from(s.clone()));
    }


    Ok(())
}
