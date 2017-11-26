use std::sync::{Arc, Mutex};
use grpcio::*;
use std::sync::mpsc::channel;
use futures::*;
use std::time::*;
use std::{thread, time};
use storage;
use storage::Result;
use storage::Store;
use operation::Looker;
use futures::sync::oneshot;
use std::sync::atomic::{AtomicBool, Ordering};
use pb;

use hyper::server::{Http, Request, Response, Service};


pub struct Server {
    ip_bind: String,
    #[allow(dead_code)]
    ip: String,
    port: u16,
    pub master_node: String,
    pub pulse_seconds: i64,
    pub data_center: String,
    pub rack: String,
    pub store: Arc<Mutex<storage::Store>>,
    pub needle_map_kind: storage::NeedleMapType,
    pub read_redirect: bool,

    handles: Vec<Option<thread::JoinHandle<()>>>,
    // signal http server quit
    shundown: Option<oneshot::Sender<()>>,
    // signal hearbeat quit
    is_stop: Arc<AtomicBool>,
}

impl Server {
    pub fn new(
        ip_bind: &str,
        ip: &str,
        port: u16,
        public_url: &str,
        folders: Vec<String>,
        max_counts: Vec<i64>,
        needle_map_kind: storage::NeedleMapType,
        master_node: &str,
        pulse_seconds: i64,
        data_center: &str,
        rack: &str,
        _white_list: Vec<String>,
        read_redirect: bool,
    ) -> Server {
        let store = storage::Store::new(ip, port, public_url, folders, max_counts, needle_map_kind);
        let server = Server {
            ip_bind: String::from(ip_bind),
            ip: String::from(ip),
            port: port,
            master_node: String::from(master_node),
            pulse_seconds: pulse_seconds,
            data_center: String::from(data_center),
            rack: String::from(rack),
            needle_map_kind: needle_map_kind,
            read_redirect: read_redirect,
            store: Arc::new(Mutex::new(store)),
            handles: vec![],
            shundown: None,
            is_stop: Arc::new(AtomicBool::new(true)),
        };

        server
    }

    pub fn stop(&mut self) {
        info!("stopping server...");
        self.shundown.take().unwrap().send(()).unwrap();
        self.is_stop.store(true, Ordering::SeqCst);
        for h in self.handles.iter_mut() {
            h.take().unwrap().join().unwrap();
            info!("join one thread");
        }
    }


    pub fn start(&mut self) {
        self.is_stop.store(false, Ordering::SeqCst);
        let (sender, receiver) = channel();
        let lookup = Arc::new(Mutex::new(Looker::new(&self.master_node)));

        let store = self.store.clone();
        let needle_map_kind = self.needle_map_kind;
        let read_redirect = self.read_redirect;
        let pulse_seconds = self.pulse_seconds as u64;
        let master_node = self.master_node.clone();

        let api_handle = thread::spawn(move || {
            let mut ctx = storage::api::Context {
                store: store,
                needle_map_kind: needle_map_kind,
                read_redirect: read_redirect,
                pulse_seconds: pulse_seconds,
                master_node: master_node.clone(),
                looker: lookup.clone(),
            };
            ctx.run(receiver);
        });

        let beat_handle = start_heartbeat(
            self.store.clone(),
            self.master_node.clone(),
            self.pulse_seconds,
            self.is_stop.clone(),
        );

        // http server
        let (tx, rx) = oneshot::channel();
        self.shundown = Some(tx);

        let mut addr_str = self.ip_bind.clone();
        addr_str.push_str(":");
        addr_str.push_str(&self.port.to_string());
        debug!("addr: {}", addr_str);
        let addr = addr_str.parse().unwrap();

        let http = storage::api::HTTPContext { sender: sender };

        let http_handle = thread::spawn(move || {
            let server = Http::new().bind(&addr, move || Ok(http.clone())).unwrap();
            server.run_until(rx.map_err(|_| ())).unwrap();
        });

        self.handles.push(Some(api_handle));
        self.handles.push(Some(http_handle));
        self.handles.push(Some(beat_handle));
    }
}


fn start_heartbeat(
    store: Arc<Mutex<Store>>,
    master_node: String,
    pulse_seconds: i64,
    is_stop: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        info!("start heartbeat....");
        if is_stop.load(Ordering::SeqCst) == true {
            info!("quit heartbeat");
            return;
        }

        do_heartbeat(
            store.clone(),
            master_node.clone(),
            pulse_seconds,
            is_stop.clone(),
        );
        warn!("heartbeat end....");
        thread::sleep(Duration::from_secs(pulse_seconds as u64));
    })
}

fn do_heartbeat(
    store: Arc<Mutex<Store>>,
    master_node: String,
    pulse_seconds: i64,
    is_stop: Arc<AtomicBool>,
) {
    let env = Arc::new(Environment::new(2));
    let channel = ChannelBuilder::new(env).connect(&master_node);
    let client = pb::zergling_grpc::ZerglingClient::new(channel);

    let (mut sink, mut receiver) = client.send_heartbeat();

    let h = thread::spawn(move || loop {
        match receiver.into_future().wait() {
            Ok((Some(beat), r)) => {
                debug!("recv: {:?}", beat);
                receiver = r;
            }
            Ok((None, _)) => {
                info!("director server close heartbeat");
                break;
            }
            Err((e, _)) => {
                error!("RPC failed: {:?}", e);
                break;
            }
        }
    });

    loop {
        if is_stop.load(Ordering::SeqCst) == true {
            // https://docs.rs/futures/0.1.16/futures/sink/trait.Sink.html#method.close
            // just close will panic ( "no Task is currently running" ), can't understand yet
            // sink.close();
            future::poll_fn(|| sink.close()).wait().unwrap();
            break;
        }
        let beat = store.lock().unwrap().collect_heartbeat();
        match sink.send((beat, WriteFlags::default())).wait() {
            Ok(ret) => sink = ret,
            Err(err) => {
                error!("send err: {}", err);
                break;
            }
        }

        thread::sleep(Duration::from_secs(pulse_seconds as u64));
    }

    if let Err(err) = h.join() {
        error!("join err: {:?}", err);
    }
}
