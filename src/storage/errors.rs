use std::result;
use std::error;
use std;


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
        ParseIntError(err: std::num::ParseIntError) {
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
