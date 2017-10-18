use std::error;
use std::result;
use std::convert::From;
use iron::IronError;
use iron::status;
use serde_json;




quick_error! {
    #[derive(Debug)]
    pub enum Error {
        GetOptionErr(t: String) {
            display("parse {} error", t)
        }
        NoFreeSpace {
            display("No free volume left")
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
