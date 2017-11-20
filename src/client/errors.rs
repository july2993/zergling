use std::result;
use std::error;
use serde_json;
use std;
use grpcio;
use util;
use hyper;
use operation;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        IO(err: std::io::Error) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        SerdeJsonError(err: serde_json::Error) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        HyperError(err: hyper::Error) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        UriError(err: hyper::error::UriError) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        Util(err: util::Error) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        Other(err: Box<error::Error + Sync + Send>) {
             from()
             cause(err.as_ref())
             description(err.description())
             display("{:?}", err)
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
