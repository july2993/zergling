// TODO:
#![allow(unused_imports)]


use std::sync::{Arc, Mutex};

use super::topology::Topology;
use super::sequencer::{Sequencer, MemorySequencer};
use grpcio::{ClientStreamingSink, RequestStream, RpcContext, RpcStatus, RpcStatusCode,UnarySink,DuplexSink};

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};


use futures::Stream;

use super::api::*;

use pb::zergling_grpc::Zergling as ZService;
use pb::zergling::*;
use directory::topology::VolumeGrow;
use storage;



pub struct Server {
    pub port: i32,
    pub meta_folder: String,
    pub default_replica_placement: storage::ReplicaPlacement,

    // pub volumeSizeLimitMB: u64,
    // pub preallocate: i64,
    // pub pulse_seconds: i64,
    pub garbage_threshold: f64,
    
    // pub topo: Topology,
    
    pub topo: Arc<Mutex<Topology>>,
    pub vg: Arc<Mutex<VolumeGrow>>,
}



impl Server {
    // TODO: add whiteList sk
    pub fn new(port: i32, meta_folder: &str, volume_size_limit_mb: u64,
               pluse_seconds: u64,
               default_replica_placement: storage::ReplicaPlacement,
               garbage_threshold: f64,
               seq: MemorySequencer) -> Server {
        let mut dir = Server {
            port: port,
            garbage_threshold: garbage_threshold,
            default_replica_placement: default_replica_placement,
            meta_folder: meta_folder.to_string(),
            topo: Arc::new(Mutex::new(Topology::new(seq, volume_size_limit_mb, pluse_seconds))),
            vg: Arc::new(Mutex::new(VolumeGrow::new())),
        };

        dir
    }


    pub fn serve(&self) {
        let ctx = Context{
            topo: self.topo.clone(),
            vg: self.vg.clone(),
            default_replica_placement: self.default_replica_placement,
        };

        let mut addr_str = "127.0.0.1:".to_string();
        addr_str.push_str(&self.port.to_string());
        let addr = addr_str.parse().unwrap();
        let server = Http::new().bind(&addr, move || Ok(ctx.clone())).unwrap();
        server.run().unwrap();

    }
}





// start grpc
// 

impl ZService for Server {
    fn send_heartbeat(&self, ctx: RpcContext,
                      stream: RequestStream<Heartbeat>, 
                      sink: DuplexSink<HeartbeatResponse>) {
        let to_send = stream
            .map(move | heartbeat| {
                let mut topo= self.topo.lock().unwrap();
                topo.sequence.set_max(heartbeat.max_file_key);
                let mut ip = heartbeat.ip.clone();
                if heartbeat.ip == "" {
                    info!("remote host is detected as: {:?}", ctx.host());
                    ip = String::from_utf8(ctx.host().to_vec()).unwrap();
                }
                
                // TODO add configuration ip -> dc_name, rack_name
                let mut dc_name = heartbeat.data_center;
                let mut rack_name = heartbeat.rack;
                if dc_name == "" {
                    dc_name = String::from("DefaultDataCenter");
                }

                if rack_name == "" {
                    rack_name = String::from("DefaultRack");
                }

                let dc = topo.get_or_create_data_center(&dc_name);
                let rack = dc.borrow_mut().get_or_create_rack(&rack_name);
                let node_name = format!("{}:{}", ip, heartbeat.port);
                let node = rack.borrow_mut().get_or_create_data_node(&node_name, &ip, heartbeat.port as i64, &heartbeat.public_url, heartbeat.max_volume_count as i64);

            });


        // let f = resp.send_all(to_send)
        //     .map(|_| {})
        //     .map_err(|e| error!("failed to route chat: {:?}", e));
        // ctx.spawn(f)

        
        
    
    }

}



//
// end grpc

