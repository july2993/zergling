
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unknown_lints)]
// #![crate_type = "lib"]
// #![cfg_attr(test, feature(test))]
#![feature(fnbox)]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate protobuf;
extern crate grpcio;
// extern crate futures_cpupool;
extern crate iron;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate log;
// extern crate env_logger;
// extern crate urlencoded;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
// extern crate serde;
extern crate rand;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;
extern crate byteorder;
extern crate time;
extern crate crc;
extern crate libflate;





pub mod directory;
#[macro_use]
pub mod util;
pub mod pb;
pub mod storage;
pub mod operation;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
