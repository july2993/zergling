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
