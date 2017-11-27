extern crate zergling;

use zergling::directory::server::Server as DServer;
use zergling::storage::Server as VServer;
use zergling::storage;
use zergling::directory;
use std::fs;
use std::thread;
use std::time::Duration;


pub struct Setter {
    dserver: DServer,
    vserver: VServer,
}


impl Setter {
    pub fn new() -> Self {
        Setter {
            dserver: get_dserver(),
            vserver: get_vserver(),
        }
    }

    pub fn start(&mut self) {
        self.dserver.start();
        self.vserver.start();
        // wait for dserver to find vserver...
        thread::sleep(Duration::new(5, 0));
    }

    pub fn stop(&mut self) {
        self.dserver.stop();
        self.vserver.stop();
    }
}

fn get_dserver() -> DServer {
    let dir = DServer::new(
        "127.0.0.1",
        "127.0.0.1",
        9333,
        "./",
        30 * 1000,
        3,
        storage::ReplicaPlacement::new("000").unwrap(),
        0.3,
        directory::sequencer::MemorySequencer::new(),
    );
    dir
}

fn get_vserver() -> VServer {
    let dir = "/tmp/zergling-test";
    let _ = fs::remove_dir_all(dir);
    let _ = fs::create_dir(dir);
    let server = VServer::new(
        "127.0.0.1",
        "127.0.0.1",
        8080,
        "127.0.0.1:8080",
        vec![dir.to_owned()],
        vec![7],
        storage::NeedleMapType::NeedleMapInMemory,
        "127.0.0.1:9334",
        3,
        "",
        "",
        vec![],
        false,
    );
    server
}
