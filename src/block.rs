use core::any::Any;

pub trait BlockDevice: Send + Sync + Any {
    fn read_block(&self, block_id: usize, buffer: &mut [u8]);
    fn write_block(&self, block_id: usize, buffer: &[u8]);
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use crate::{
        block::BlockDevice,
        cache::{get_block_cache, BLOCK_SIZE},
    };
    use alloc::sync::Arc;
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom, Write},
        sync::Condvar,
    };
    use xx_mutex_lock::Mutex;

    // 模拟硬盘驱动
    struct BlockFile(Mutex<File>);
    impl BlockDevice for BlockFile {
        fn read_block(&self, block_id: usize, buf: &mut [u8]) {
            let mut file = self.0.lock();
            file.seek(SeekFrom::Start((block_id * BLOCK_SIZE) as u64))
                .expect("Error when seeking!");
            assert_eq!(file.read(buf).unwrap(), BLOCK_SIZE, "Not a complete block!");
        }

        fn write_block(&self, block_id: usize, buf: &[u8]) {
            let mut file = self.0.lock();
            file.seek(SeekFrom::Start((block_id * BLOCK_SIZE) as u64))
                .expect("Error when seeking!");
            assert_eq!(
                file.write(buf).unwrap(),
                BLOCK_SIZE,
                "Not a complete block!"
            );
        }
    }

    impl Drop for BlockFile {
        fn drop(&mut self) {
            let _ = self.0.lock().sync_all();
        }
    }
}
