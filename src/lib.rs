#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![feature(fnbox)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate protobuf;
extern crate grpcio;
extern crate iron;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prometheus;
extern crate rand;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;
extern crate byteorder;
extern crate time;
extern crate crc;
extern crate libflate;
extern crate multipart;
extern crate mime_guess;
extern crate lru;
extern crate mime;
extern crate serde;
extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate signal;
extern crate nix;
extern crate futures_cpupool;

pub mod directory;
#[macro_use]
pub mod util;
pub mod pb;
pub mod storage;
pub mod client;
pub mod operation;
pub mod metrics;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
