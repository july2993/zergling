
use std::fs::File;


pub trait NeedleMapper {
    fn put(&mut self, key: u64, offset: u32, size: u32) -> bool;
    fn get(&mut self, key: u64) -> Option<NeedleValue>;
    fn delete(&mut self, key: u64, offset: u32) -> bool;

    
}


pub struct baseNeedleMapper struct {
    indexFile: File,


    DeletionCounter: u64,
    FileCounter: u64,
    DeletionByteCounter: u64,
    FileByteCounter: u64,
    MaximumFileKey u64,
}


pub struct NeedleMap {

}
