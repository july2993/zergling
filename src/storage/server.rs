use std::sync::{Arc, Mutex};
use grpcio::*;
use std::sync::mpsc::channel;
// use std::sync::mpsc::sync_channel;
use futures::*;
use std::time::*;
use std::{thread, time};
use storage;
use storage::Result;
use storage::Store;
use pb;

use hyper::server::{Http, Request, Response, Service};


pub struct Server {
    ip: String,
    port: u16,
    pub master_node: String,
    pub pulse_seconds: i64,
    pub data_center: String,
    pub rack: String,
    pub store: Arc<Mutex<storage::Store>>,

    pub needle_map_kind: storage::NeedleMapType,
    pub fix_jpg_orientation: bool,
    pub read_redirect: bool,
}

impl Server {
    pub fn new(
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
        fix_jpg_orientation: bool,
        read_redirect: bool,
    ) -> Server {
        let store = storage::Store::new(ip, port, public_url, folders, max_counts, needle_map_kind);
        let server = Server {
            ip: String::from(ip),
            port: port,
            master_node: String::from(master_node),
            pulse_seconds: pulse_seconds,
            data_center: String::from(data_center),
            rack: String::from(rack),
            needle_map_kind: needle_map_kind,
            fix_jpg_orientation: fix_jpg_orientation,
            read_redirect: read_redirect,
            store: Arc::new(Mutex::new(store)),
        };

        server
    }

    pub fn spawn_heartbeat(&self, ctx: storage::api::Context) {
        thread::spawn(move || ctx.heartbeat());
    }


    pub fn serve(&self) {
        let (sender, receiver) = channel();
        let ctx = storage::api::Context {
            sender: sender.clone(),
            store: self.store.clone(),
            needle_map_kind: self.needle_map_kind,
            read_redirect: self.read_redirect,
            pulse_seconds: self.pulse_seconds as u64,
            master_node: self.master_node.clone(),
        };


        let store = self.store.clone();
        let needle_map_kind = self.needle_map_kind;
        let read_redirect = self.read_redirect;
        let pulse_seconds = self.pulse_seconds as u64;
        let master_node = self.master_node.clone();

        thread::spawn(move || {
            let mut ctx = storage::api::Context {
                sender: sender.clone(),
                store: store,
                needle_map_kind: needle_map_kind,
                read_redirect: read_redirect,
                pulse_seconds: pulse_seconds,
                master_node: master_node.clone(),
            };
            ctx.run(receiver)
        });

        self.spawn_heartbeat(ctx.clone());

        let mut addr_str = self.ip.clone();
        addr_str.push_str(":");
        addr_str.push_str(&self.port.to_string());
        debug!("addr: {}", addr_str);
        let addr = addr_str.parse().unwrap();
        let server = Http::new().bind(&addr, move || Ok(ctx.clone())).unwrap();
        server.run().unwrap();
    }
}
