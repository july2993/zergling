use super::NeedleValue;
use std::collections::HashMap;


pub trait NeedleValueMap: Send {
    // return old needle if has
    fn set(&mut self, key: u64, needle_value: NeedleValue) -> Option<NeedleValue>;
    fn delete(&mut self, key: u64) -> Option<NeedleValue>;
    fn get(&self, key: u64) -> Option<NeedleValue>;
}


// #[derive(Default)]
pub struct MemNeedleValueMap {
    hm: HashMap<u64, NeedleValue>,
}

impl MemNeedleValueMap {
    pub fn new() -> MemNeedleValueMap {
        MemNeedleValueMap { hm: HashMap::new() }
    }
}


impl NeedleValueMap for MemNeedleValueMap {
    fn set(&mut self, key: u64, needle_value: NeedleValue) -> Option<NeedleValue> {
        self.hm.insert(key, needle_value)
    }
    fn delete(&mut self, key: u64) -> Option<NeedleValue> {
        self.hm.remove(&key)
    }
    fn get(&self, key: u64) -> Option<NeedleValue> {
        self.hm.get(&key).map(|v| v.clone())
    }
}
