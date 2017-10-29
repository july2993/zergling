use std::error;
use std::result;
use std::convert::From;
use iron::IronError;
use iron::status;
use serde_json;
use url;
use storage;
use std;




quick_error! {
    #[derive(Debug)]
    pub enum Error {
        GetOptionErr(t: String) {
            display("parse {} error", t)
        }
        NoFreeSpace(t: String) {
            display("no free space: {}", t)
        }
        NoWritableVolume(msg: String) {
            display("No more writable volume: {}", msg)
        }
        Other(err: Box<error::Error + Sync + Send>) {
            from()
            cause(err.as_ref())
            description(err.description())
            display("{:?}", err)
        }
        String(s: String){
            from()
            description(s)
            display("{:?}", s)
        }
        SerdeJson(err: serde_json::Error) {
            from()
            cause(err)
            description(err.description())
            display("{:?}", err)
        }
        UrlParseError(err: url::ParseError) {
            from()
            cause(err)
            description(err.description())
            display("{:?}", err)
        }
        StorageError(err: storage::Error) {
            from()
            cause(err)
            description(err.description())
            display("{:?}", err)
        }
        ParseIntError(err: std::num::ParseIntError) {
            from()
            cause(err)
            description(err.description())
            display("{:?}", err)
        }
    }
}
pub type Result<T> = result::Result<T, Error>;



// the trait `std::convert::From<directory::errors::Error>` is not implemented for `iron::IronError`

impl From<Error> for IronError {
    fn from(err: Error) -> IronError {
        IronError::new(err, status::BadRequest)
    }
}

// impl From<serde_json::Error> for IronError {
//     fn from(err: serde_json::Error) -> IronError {
//         IronError::new(err, status::BadRequest)
//     }
// }
