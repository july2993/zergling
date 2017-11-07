#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(unused_mut)]
#![allow(unused_imports)]

extern crate zergling;
// extern crate env_logger;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
extern crate chrono;


// use log::Level;
use clap::{App, Arg, ArgMatches, SubCommand};
use env_logger::LogBuilder;
use chrono::Local;
use std::env;


use zergling::storage;
use zergling::directory::server::Server;
use zergling::storage::Server as VServer;
use zergling::directory::sequencer::MemorySequencer;
use zergling::storage::NeedleMapType;


fn main() {
    LogBuilder::new()
        .format(|record| {
            format!(
                "{} [{}:{}] - {} {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.location().file().rsplit('/').nth(0).unwrap(),
                record.location().line(),
                record.level(),
                record.args()
            )
        })
        .parse(&env::var("ZERGLING_LOG").unwrap_or_default())
        .init()
        .unwrap();

    // #[warn(unused_must_use)]
    // env_logger::init();


    debug!("this is printed by default");
    info!("this is printed by default");
    warn!("this is printed by default");
    error!("this is printed by default");

    let matches = App::new("zergling")
        .arg(Arg::with_name("ip").long("ip").takes_value(true).help(
            "ip address default localhost",
        ))
        .arg(
            Arg::with_name("ip.bind")
                .long("ip.bind")
                .takes_value(true)
                .help("default 0.0.0.0"),
        )
        .arg(Arg::with_name("port").long("port").takes_value(true))
        .arg(Arg::with_name("mdir").long("mdir").takes_value(true))
        .subcommand(SubCommand::with_name("master").about("master server"))
        .subcommand(SubCommand::with_name("volume").about("volume server"))
        .get_matches();

    let mut ip = "127.0.0.1";
    let mut ip_bind = "0.0.0.0";
    let mut port = 9333;
    let mut volume_size_limit_mb = 30 * 1000;
    let mut replica_placement = storage::ReplicaPlacement::new("000").unwrap();
    let mut pluse = 5;
    let mut garbage_threshold = 0.3;

    // TODO change default
    let mut mdir = "./";

    if let Some(c) = matches.value_of("ip") {
        ip = c
    }
    if let Some(c) = matches.value_of("ip.bind") {
        ip_bind = c
    }
    if let Some(c) = matches.value_of("port") {
        port = c.parse().unwrap();
        println!("port {}", port);
    }
    if let Some(c) = matches.value_of("mdir") {
        mdir = c
    }

    if let Some(_matches) = matches.subcommand_matches("master") {
        println!("starting master server[{}]....", port);

        let seq = MemorySequencer::new();

        let dir = Server::new(
            ip,
            port,
            mdir,
            volume_size_limit_mb,
            pluse,
            replica_placement,
            garbage_threshold,
            seq,
        );
        dir.serve(ip_bind);

    }

    if let Some(_matches) = matches.subcommand_matches("volume") {
        println!("starting volumn server....");

        let server = VServer::new(
            ip,
            port,
            "127.0.0.1:8080",
            vec!["./vdata".to_owned()],
            vec![7],
            NeedleMapType::NeedleMapInMemory,
            // TODO config master node
            "127.0.0.1:9334",
            // todo
            100,
            "",
            "",
            vec![],
            true,
            true,
        );
        server.serve();
    }
}
