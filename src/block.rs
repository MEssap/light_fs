use core::any::Any;

pub trait BlockDevice: Send + Sync + Any {
    fn read_block(&self, block_id: usize, buffer: &mut [u8]);
    fn write_block(&self, block_id: usize, buffer: &[u8]);
}
