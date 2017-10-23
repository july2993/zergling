
extern crate router;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use router::Router;
use super::topology::Topology;
use super::sequencer::{Sequencer, MemorySequencer};
use super::sequencer;
use grpcio::{ClientStreamingSink, RequestStream, RpcContext, RpcStatus, RpcStatusCode,UnarySink,DuplexSink};

use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};



use super::api::*;

use pb::zergling_grpc::Zergling as ZService;
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

        }
}



//
// end grpc

