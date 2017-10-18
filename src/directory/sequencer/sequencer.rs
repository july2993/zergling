

pub trait Sequencer {
    fn NextFileId(&self, count: u64) -> (u64, u64);
    fn SetMax(&self, u64);
    fn Peek(&self) -> u64;
}
