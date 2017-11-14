use std::result;
use std::error;
use serde_json;
use std;
use grpcio;
use operation;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ParseReplicaPlacement(t: String) {
            display("parse {} error", t)
        }
        ParseTTL(t: String) {
            display("parse {} error", t)
        }
        IO(err: std::io::Error) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        String(s: String) {
            from()
            description(s)
            display("{:?}", s)
        }
        ParseIntError(err: std::num::ParseIntError) {
            from()
            cause(err)
            display("{:?}", err)
            description(err.description())
        }
        GrpcIOError(err: grpcio::Error) {
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
        Operation(err: operation::Error) {
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
