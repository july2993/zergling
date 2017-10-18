
extern crate iron;
extern crate router;

use std::sync::{Arc, Mutex};

use iron::prelude::*;
use iron::status;
use iron::response::Response;
use router::Router;
use super::topology::Topology;
use super::sequencer::{Sequencer, MemorySequencer};
use super::sequencer;
use grpcio::{ClientStreamingSink, RequestStream, RpcContext, RpcStatus, RpcStatusCode,UnarySink,DuplexSink};
use super::api::*;

use pb::zergling_grpc::Zergling as Service;
use pb::zergling::*;
use directory::topology::VolumeGrow;



pub struct Server {
    pub port: i32,
    pub metaFolder: String,

    // pub volumeSizeLimitMB: u64,
    // pub preallocate: i64,
    // pub pulseSeconds: i64,
    // pub defaultReplicaPlacement: String,
    // pub garbageThreshold: String,
    
    // pub topo: Topology,
    
    pub topo: Arc<Mutex<Topology>>,
    pub vg: Arc<Mutex<VolumeGrow>>,
}



impl Server {
    // TODO: add whiteList sk
    pub fn new(port: i32, metaFolder: &str, volumeSizeLimitMB: u64,
               pluseSeconds: u64,
               defaultReplicaPlacement: String,
               garbageThreshold: f64,
               seq: MemorySequencer) -> Server {
        let mut dir = Server {
            port: port,
            metaFolder: metaFolder.to_string(),
            topo: Arc::new(Mutex::new(Topology::new(seq, volumeSizeLimitMB, pluseSeconds))),
            vg: Arc::new(Mutex::new(VolumeGrow::new())),
        };

        dir
    }


    pub fn serve(&self) {
        let mut router = Router::new();


        // https://stackoverflow.com/questions/38915653/how-can-i-pass-around-variables-between-handlers
        let ctx = Context{
            topo: self.topo.clone(),
            vg: self.vg.clone(),
        };
        {
            let c = ctx.clone();
            router.get("/:test_handler", move |request: &mut Request| test_handler(request, &c), "test");
        }

        {
            let c = ctx.clone();
            router.get("/dir/assign", move |request: &mut Request| assign_handler(request, &c), "assign");
        }


        Iron::new(router).http("localhost:".to_string() + &self.port.to_string()).unwrap();
    }

}




// start grpc
// 

impl Service for Server {
    fn send_heartbeat(&self, ctx: RpcContext,
                      stream: RequestStream<Heartbeat>, 
                      sink: DuplexSink<HeartbeatResponse>) {

        }
}



//
// end grpc

