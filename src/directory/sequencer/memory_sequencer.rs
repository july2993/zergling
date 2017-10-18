use std::sync::{Arc, Mutex};
use super::Sequencer;


#[derive(Debug)]
pub struct MemorySequencer {
    counter: Arc<Mutex<u64>>,
}

impl MemorySequencer {
    pub fn new() -> MemorySequencer {
       MemorySequencer {
           counter: Arc::new(Mutex::new(1)),
       } 
    }
}


impl Sequencer for MemorySequencer {
    fn NextFileId(&self, count: u64) -> (u64, u64) {
        let mut counter = self.counter.lock().unwrap();
        let ret = *counter;
        *counter += count;
        (ret, count)
    }

    fn SetMax(&self, seenValue: u64) {
        let mut counter = self.counter.lock().unwrap();
        if *counter <= seenValue {
            *counter = seenValue;
        }
    }

    fn Peek(&self) -> u64 {
        let mut counter = self.counter.lock().unwrap();
        
        *counter
    }
}
