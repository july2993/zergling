use std::result;
use std::error;


quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Other(err: Box<error::Error + Sync + Send>) {
             from()
             cause(err.as_ref())
             description(err.description())
             display("{:?}", err)
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
