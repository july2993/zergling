use std::sync::{Arc, Mutex};
use grpcio::*;
use std::sync::mpsc::channel;
// use std::sync::mpsc::sync_channel;
use futures::*;
use std::thread;
use storage;

use hyper::server::{Http, Request, Response, Service};


pub struct Server {
    ip: String,
    port: u16,
    pub master_node: String,
    pub pulse_seconds: i64,
    pub data_center: String,
    pub rack: String,
    pub store: storage::Store,


    pub needle_map_kind: storage::NeedleMapType,
    pub fix_jpg_orientation: bool,
    pub read_redirect: bool,
}

impl Server {
    pub fn new(ip: &str, port: u16, public_url: &str, folders: Vec<String>
               , max_counts: Vec<i64>, needle_map_kind: storage::NeedleMapType,
               master_node: &str, pulse_seconds: i64, data_center: &str, rack: &str,
               white_list: Vec<String>, fix_jpg_orientation: bool, read_redirect: bool) -> Server {
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
            store: store,
        };

        server
    }


    pub fn serve(&self, ) {

        let (sender, receiver) = channel();
        let ctx = storage::api::Context{ sender: Arc::new(sender.clone()) };

        let ctx_o = ctx.clone();
        thread::spawn(move || {
            let mut ctx = storage::api::Context{ sender: Arc::new(sender.clone()) };
            ctx.run(receiver)
        });

        let mut addr_str = self.ip.clone();
        addr_str.push_str(":");
        addr_str.push_str(&self.port.to_string());
        let addr = addr_str.parse().unwrap();
        let server = Http::new().bind(&addr, move || Ok(ctx.clone())).unwrap();
        server.run().unwrap();
        
    }

}
