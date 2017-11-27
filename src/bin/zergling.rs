#![feature(plugin)]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate nix;
extern crate signal;
extern crate zergling;


use clap::{App, Arg, SubCommand};
use std::str::FromStr;
use zergling::storage;
use zergling::directory::server::Server;
use zergling::storage::Server as VServer;
use zergling::directory::sequencer::MemorySequencer;
use zergling::storage::NeedleMapType;
use zergling::util;
use signal::trap::Trap;
use nix::sys::signal::{SIGUSR1, SIGUSR2, SIGHUP, SIGINT, SIGTERM};


fn main() {
    util::init_log();

    let matches = App::new("zergling")
        .arg(
            Arg::with_name("ip.bind")
                .long("ip.bind")
                .takes_value(true)
                .help("default 0.0.0.0"),
        )
        .subcommand(
            SubCommand::with_name("master")
                .about("master server")
                .arg(
                    Arg::with_name("ip")
                        .long("ip")
                        .takes_value(true)
                        .help("ip address default localhost"),
                )
                .arg(Arg::with_name("mdir").long("mdir").takes_value(true))
                .arg(Arg::with_name("port").long("port").takes_value(true))
                .arg(
                    Arg::with_name("pulse_seconds")
                        .long("pulse_seconds")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("volume_size_limit_mb")
                        .long("volume_size_limit_mb")
                        .takes_value(true)
                        .long_help(
                            "Master stops directing writes to oversized volumes. default 30000",
                        ),
                )
                .arg(
                    Arg::with_name("default_replication")
                        .long("default_replication")
                        .takes_value(true)
                        .long_help("Default replicattion if not specified.")
                        .default_value("000"),
                ),
        )
        .subcommand(
            SubCommand::with_name("volume")
                .about("volume server")
                .arg(
                    Arg::with_name("pulse_seconds")
                        .long("pulse_seconds")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ip")
                        .long("ip")
                        .takes_value(true)
                        .help("ip address default localhost"),
                )
                .arg(Arg::with_name("port").long("port").takes_value(true))
                .arg(
                    Arg::with_name("public_url")
                        .long("public_url")
                        .takes_value(true)
                        .long_help("public access url"),
                )
                .arg(
                    Arg::with_name("data_center")
                        .long("data_center")
                        .takes_value(true)
                        .long_help("data_center")
                        .default_value(""),
                )
                .arg(
                    Arg::with_name("rack")
                        .long("rack")
                        .takes_value(true)
                        .long_help("rack")
                        .default_value(""),
                )
                .arg(
                    Arg::with_name("dir")
                        .long("dir")
                        .takes_value(true)
                        .multiple(true)
                        .long_help(
                            "directories to store data files. -dir dir_name:max_volume_counts
                                   like -dir /data1:7 max_volume_counts default 7 if not specified",
                        ),
                )
                .arg(
                    Arg::with_name("master_server")
                        .long("master_server")
                        .takes_value(true)
                        .long_help("master server location")
                        .default_value("localhost:9334"),
                ),
        )
        .get_matches();

    if let Some(_matches) = matches.subcommand_matches("master") {
        let mdir = _matches.value_of("mdir").unwrap_or("./");
        let ip = _matches.value_of("ip").unwrap_or("127.0.0.1");
        let ip_bind = _matches.value_of("ip.bind").unwrap_or("0.0.0.0");
        let port: u16 = _matches
            .value_of("port")
            .map(|s| s.parse().unwrap())
            .unwrap_or(9333);
        let pluse = _matches
            .value_of("pulse_seconds")
            .map(|s| s.parse().unwrap())
            .unwrap_or(5);
        let volume_size_limit_mb = _matches
            .value_of("volume_size_limit_mb")
            .map(|s| s.parse().unwrap())
            .unwrap_or(30 * 1000);
        let s = _matches.value_of("default_replication").unwrap();
        let replica_placement = storage::ReplicaPlacement::new(s).unwrap();

        println!("starting master server[{}]....", port);

        let garbage_threshold = 0.3;

        let seq = MemorySequencer::new();

        let mut dir = Server::new(
            ip_bind,
            ip,
            port,
            mdir,
            volume_size_limit_mb,
            pluse,
            replica_placement,
            garbage_threshold,
            seq,
        );
        dir.start();
        handle_signal();
        dir.stop();
    }

    if let Some(_matches) = matches.subcommand_matches("volume") {
        println!("starting volumn server....");
        let ip_bind = _matches.value_of("ip.bind").unwrap_or("0.0.0.0");
        let ip = _matches.value_of("ip").unwrap_or("127.0.0.1");
        let pluse = _matches
            .value_of("pulse_seconds")
            .map(|s| s.parse().unwrap())
            .unwrap_or(5);
        let port: u16 = _matches
            .value_of("port")
            .map(|s| s.parse().unwrap())
            .unwrap_or(9333);

        let public_url = _matches
            .value_of("public_url")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let mut s = String::from_str(ip).unwrap();
                s.push_str(":");
                s.push_str(&port.to_string());

                s
            });

        let dirs: Vec<&str> = _matches.values_of("dir").unwrap().collect();
        let paths: Vec<String> = dirs.iter()
            .map(|x| match x.rfind(":") {
                Some(idx) => x[0..idx].to_string(),
                None => x.to_string(),
            })
            .collect();

        let max_volumes = dirs.iter()
            .map(|x| match x.rfind(":") {
                Some(idx) => x[idx + 1..].parse::<i64>().unwrap(),
                None => 7,
            })
            .collect();


        let master_server = _matches.value_of("master_server").unwrap();
        let data_center = _matches.value_of("data_center").unwrap();
        let rack = _matches.value_of("rack").unwrap();

        let mut server = VServer::new(
            ip_bind,
            ip,
            port,
            &public_url,
            paths,
            max_volumes,
            NeedleMapType::NeedleMapInMemory,
            master_server,
            pluse,
            data_center,
            rack,
            vec![],
            false,
        );
        server.start();
        handle_signal();
        server.stop();
    }
}


fn handle_signal() {
    let trap = Trap::trap(&[SIGTERM, SIGINT, SIGHUP, SIGUSR1, SIGUSR2]);

    for sig in trap {
        match sig {
            SIGTERM | SIGINT | SIGHUP => {
                info!("receive signal {:?}, stopping server...", sig);
                break;
            }
            _ => unreachable!(),
        }
    }
}
