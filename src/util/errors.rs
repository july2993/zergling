use std::result;
use std::error;
use serde_json;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        SerdeJsonError(err: serde_json::Error) {
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
