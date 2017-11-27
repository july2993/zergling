#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
#![feature(fnbox)]
#![feature(custom_attribute)]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate byteorder;
extern crate chrono;
extern crate crc;
extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate grpcio;
extern crate hyper;
extern crate iron;
#[macro_use]
extern crate lazy_static;
extern crate libflate;
#[macro_use]
extern crate log;
extern crate lru;
extern crate mime;
extern crate mime_guess;
extern crate multipart;
extern crate nix;
#[macro_use]
extern crate prometheus;
extern crate protobuf;
#[macro_use]
extern crate quick_error;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate signal;
extern crate time;
extern crate tokio_core;
extern crate url;

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
