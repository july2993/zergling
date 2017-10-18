extern crate zergling;
// extern crate env_logger;
#[macro_use]
extern crate log;
extern crate clap;

// use log::Level;
use clap::{App, Arg, ArgMatches, SubCommand};


use zergling::directory::server::Server;
use zergling::directory::sequencer::MemorySequencer;


fn main() {
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    let matches = App::new("zergling")
        .arg(
            Arg::with_name("ip")
            .long("ip")
            .takes_value(true)
            .help("ip address default localhost")
            )
        .arg(
            Arg::with_name("ip.bind")
            .long("ip.bind")
            .takes_value(true)
            .help("default 0.0.0.0")
            )
        .arg(
            Arg::with_name("port")
            .long("port")
            .takes_value(true)
            )
        .arg(
            Arg::with_name("mdir")
            .long("mdir")
            .takes_value(true)
            )
        .subcommand(SubCommand::with_name("master")
                    .about("master server"))
        .subcommand(SubCommand::with_name("volumn")
                    .about("volumn server"))
        .get_matches();

    let mut ip = "localhost";
    let mut ip_bind = "0.0.0.0";
    let mut port = 9334;
    let mut volumeSizeLimitMB = 30*1000;
    let mut replicaPlacement = String::from("000");
    let mut pluse = 5;
    let mut garbageThreshold = 0.3;

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

    if let Some(matches) = matches.subcommand_matches("master") {
        println!("starting master server[{}]....", port);

        let seq = MemorySequencer::new();

        let dir = Server::new(port, mdir,
                              volumeSizeLimitMB,
                              pluse,
                              replicaPlacement,
                              garbageThreshold,
                              seq);
        dir.serve();

    }

    if let Some(matches) = matches.subcommand_matches("volumn") {
        println!("starting volumn server....");

    }


}
