use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use grpcio::*;
use futures::*;
use grpcio::Error as GError;
use pb::zergling_grpc;
use super::topology::Topology;
use super::sequencer::{MemorySequencer, Sequencer};
use grpcio::{self, ClientStreamingSink, DuplexSink, RequestStream, RpcContext, RpcStatus,
             RpcStatusCode, UnarySink};
use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use std::thread;
use futures_cpupool::CpuPool;
use futures::Stream;
use futures::sync::oneshot;
use super::api::*;
use pb;
use pb::zergling_grpc::Zergling as ZService;
use pb::zergling::*;
use directory::topology::VolumeGrow;
use storage;
use std::cell::Cell;

// #[derive(Clone)]
pub struct Server {
    pub bind_ip: String,
    pub ip: String,
    pub port: u16,
    pub meta_folder: String,
    pub default_replica_placement: storage::ReplicaPlacement,
    pub volume_size_limit_mb: u64,
    // pub preallocate: i64,
    // pub pulse_seconds: i64,
    pub garbage_threshold: f64,
    pub topo: Arc<Mutex<Topology>>,
    pub vg: Arc<Mutex<VolumeGrow>>,
    pub grpc_server: grpcio::Server,

    shundown: Option<oneshot::Sender<()>>,
    handles: Vec<Option<thread::JoinHandle<()>>>,
}

impl Server {
    // TODO: add whiteList sk
    pub fn new(
        bind_ip: &str,
        ip: &str,
        port: u16,
        meta_folder: &str,
        volume_size_limit_mb: u64,
        pluse_seconds: u64,
        default_replica_placement: storage::ReplicaPlacement,
        garbage_threshold: f64,
        seq: MemorySequencer,
    ) -> Server {
        let topo = Arc::new(Mutex::new(Topology::new(
            seq,
            volume_size_limit_mb * 1024 * 1024,
            pluse_seconds,
        )));

        let env = Arc::new(Environment::new(2));
        let service = zergling_grpc::create_zergling(GrpcServer {
            volume_size_limit_mb: volume_size_limit_mb,
            topo: topo.clone(),
        });
        let server = ServerBuilder::new(env)
            .register_service(service)
            .bind(bind_ip, port + 1)
            .build()
            .unwrap();

        let dir = Server {
            bind_ip: bind_ip.to_owned(),
            ip: String::from(ip),
            volume_size_limit_mb: volume_size_limit_mb,
            port: port,
            garbage_threshold: garbage_threshold,
            default_replica_placement: default_replica_placement,
            meta_folder: meta_folder.to_string(),
            vg: Arc::new(Mutex::new(VolumeGrow::new())),
            topo: topo.clone(),
            shundown: None,
            grpc_server: server,
            handles: vec![],
        };

        dir
    }

    pub fn stop(&mut self) {
        self.grpc_server.shutdown();
        self.shundown.take().unwrap().send(()).unwrap();
        for h in self.handles.iter_mut() {
            h.take().unwrap().join().unwrap();
        }
    }

    pub fn start(&mut self) {
        let (sender, receiver) = channel();
        self.grpc_server.start();

        let mut ctx = Context {
            topo: self.topo.clone(),
            vg: self.vg.clone(),
            default_replica_placement: self.default_replica_placement,
            ip: self.ip.clone(),
            port: self.port,
            cpu_pool: CpuPool::new(16),
        };

        let api_handle = thread::spawn(move || {
            ctx.run(receiver);
        });
        self.handles.push(Some(api_handle));


        // http server
        let http = super::api::HTTPContext { sender: sender };
        let (tx, rx) = oneshot::channel();
        self.shundown = Some(tx);

        let mut addr_str = self.bind_ip.clone();
        addr_str.push_str(":");
        addr_str.push_str(&self.port.to_string());
        let addr = addr_str.parse().unwrap();

        // server.run().unwrap();
        let handle = thread::spawn(move || {
            let server = Http::new().bind(&addr, move || Ok(http.clone())).unwrap();
            server.run_until(rx.map_err(|_| ())).unwrap();
        });

        self.handles.push(Some(handle));
    }
}


// start grpc
//
#[derive(Clone)]
struct GrpcServer {
    pub volume_size_limit_mb: u64,
    pub topo: Arc<Mutex<Topology>>,
}

impl ZService for GrpcServer {
    fn send_heartbeat(
        &self,
        ctx: RpcContext,
        stream: RequestStream<Heartbeat>,
        sink: DuplexSink<HeartbeatResponse>,
    ) {
        // TODO unregister node
        let volume_size_limit_mb = self.volume_size_limit_mb;
        let topo = self.topo.clone();
        let host = ctx.host().to_vec();

        error!("recv send_heartbeat");

        let to_send = stream
            .map(move |heartbeat| {
                // debug!("recv heartbeat: {:?}", heartbeat);

                let mut topo = topo.lock().unwrap();
                topo.sequence.set_max(heartbeat.max_file_key);
                let mut ip = heartbeat.ip.clone();
                if heartbeat.ip == "" {
                    info!("remote host is detected as: {:?}", host);
                    ip = String::from_utf8(host.to_vec()).unwrap();
                }

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
                rack.borrow_mut().data_center = Arc::downgrade(&dc);
                let node_name = format!("{}:{}", ip, heartbeat.port);
                let node = rack.borrow_mut().get_or_create_data_node(
                    &node_name,
                    &ip,
                    heartbeat.port as i64,
                    &heartbeat.public_url,
                    heartbeat.max_volume_count as i64,
                );
                node.borrow_mut().rack = Arc::downgrade(&rack);


                let mut infos = vec![];
                for info_msg in heartbeat.volumes.iter() {
                    match storage::VolumeInfo::new(info_msg) {
                        Ok(info) => infos.push(info),
                        Err(err) => info!("fail to convert joined volume: {}", err),
                    };
                }

                let deleted_volumes = node.borrow_mut().update_volumes(infos.clone());

                for v in infos {
                    topo.register_volume_layout(v, node.clone());
                }

                for v in deleted_volumes.iter() {
                    topo.un_register_volume_layout(v.clone(), node.clone());
                }

                let mut resp = pb::zergling::HeartbeatResponse::new();
                resp.volumeSizeLimit = volume_size_limit_mb;

                stream::iter_ok::<_, GError>(vec![(resp, WriteFlags::default())])
            })
            .flatten();

        let f = sink.send_all(to_send)
            .map(|_| {})
            .map_err(|e| error!("failed to send heartbeat response: {:?}", e));
        ctx.spawn(f)
    }
}
//
// end grpc
