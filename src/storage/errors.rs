use std::error;
use std::result;




quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ParseReplicaPlacement(t: String) {
            display("parse {} error", t)
        }
        ParseTTL(t: String) {
            display("parse {} error", t)
        }
    }
}


pub type Result<T> = result::Result<T, Error>;
