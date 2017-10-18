

use super::NeedleValue;


pub trait NeedleValueMap {
    fn set(key: u64, offset: u32, size: u32) -> (oldOffset, oldSize u32);
    fn delete(key: u64) -> u32;
    fn get(key: u64) -> Option<NeedleValue>;
}
