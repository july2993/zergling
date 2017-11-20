
extern crate zergling;

use zergling::directory::server::Server as DServer;
use zergling::storage::Server as VServer;
use zergling::storage;
use zergling::directory;
use std::thread;
use std::fs;
use std::time;


pub fn setup() {
    thread::spawn(|| { setup_dir(); });
    thread::spawn(|| { setup_volume(); });

    // wait to setup, should be change to a better way
    thread::sleep(time::Duration::new(5, 0));
}


pub fn setup_dir() {
    let dir = DServer::new(
        "127.0.0.1",
        9333,
        "./",
        30 * 1000,
        3,
        storage::ReplicaPlacement::new("000").unwrap(),
        0.3,
        directory::sequencer::MemorySequencer::new(),
    );
    dir.serve("127.0.0.1");
}

pub fn setup_volume() {
    let dir = "/tmp/zergling-test";
    let _ = fs::remove_dir_all(dir);
    let _ = fs::create_dir(dir);
    let server = VServer::new(
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
    server.serve();
}
