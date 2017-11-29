extern crate tempdir;
extern crate zergling;

use zergling::directory::server::Server as DServer;
use zergling::storage::Server as VServer;
use zergling::storage;
use zergling::directory;
use self::tempdir::TempDir;
use std::thread;
use std::time::Duration;

pub struct Setter {
    dir_addr: String,
    dserver: DServer,
    vserver: Vec<VServer>,

    dirs: Vec<tempdir::TempDir>,
}

impl Setter {
    pub fn new(port: u16) -> Self {
        let bind_ip = "0.0.0.0";
        Setter {
            dserver: get_dserver(bind_ip, port),
            vserver: vec![],
            dirs: vec![],
            dir_addr: format!("{}:{}", bind_ip, port),
        }
    }

    pub fn dir_addr(&self) -> String {
        self.dir_addr.clone()
    }

    pub fn add_vserver(&mut self, port: u16) {
        let dir = TempDir::new("zerglin_test").expect("create temp dir");
        let dir_addr = self.dir_addr();
        self.vserver
            .push(get_vserver(&dir_addr, port, dir.path().to_str().unwrap()));
        // let it delete dir if setter is drop
        self.dirs.push(dir);
    }

    pub fn start(&mut self) {
        self.dserver.start();
        for s in self.vserver.iter_mut() {
            s.start();
        }
        // wait for dserver to find vserver...
        thread::sleep(Duration::new(5, 0));
    }

    pub fn stop(&mut self) {
        self.dserver.stop();
        for s in self.vserver.iter_mut() {
            s.stop();
        }
    }
}

fn get_dserver(bind_ip: &str, port: u16) -> DServer {
    let dir = DServer::new(
        bind_ip,
        "127.0.0.1",
        port,
        "./",
        30 * 1000,
        3,
        storage::ReplicaPlacement::new("000").unwrap(),
        0.3,
        directory::sequencer::MemorySequencer::new(),
    );
    dir
}

fn get_vserver(dir_addr: &str, port: u16, dir: &str) -> VServer {
    // let _ = fs::remove_dir_all(dir);
    // let _ = fs::create_dir(dir);
    let server = VServer::new(
        "127.0.0.1",
        "127.0.0.1",
        port,
        "127.0.0.1:8080",
        vec![dir.to_owned()],
        vec![700],
        storage::NeedleMapType::NeedleMapInMemory,
        dir_addr,
        3,
        "",
        "",
        vec![],
        false,
    );
    server
}
