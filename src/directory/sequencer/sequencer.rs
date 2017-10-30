

pub trait Sequencer {
    fn next_file_id(&self, count: u64) -> (u64, u64);
    fn set_max(&self, u64);
    fn peek(&self) -> u64;
}
