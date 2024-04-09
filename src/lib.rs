pub mod block;
pub mod cache;
pub mod easy_fs;
pub mod ext4;
pub mod fat32;
pub mod file;

use crate::cache::BlockCacheManager;
use xx_mutex_lock::Mutex;

pub static BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> = Mutex::new(BlockCacheManager::new());

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

    #[test]
    fn tests() {
        let block_file: Arc<BlockFile> = Arc::new(BlockFile(Mutex::new({
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open("./efs.img")
                .expect("cannot open img");
            f.set_len(100 * 1024 * 1024).unwrap();
            f
        })));

        let mut buf = [0u8; 512];
        let buf0 = [0u8; 512];
        let buf1 = [1u8; 512];

        block_file.read_block(0, &mut buf);
        assert_eq!(buf, buf0);

        block_file.write_block(0, &buf1);
        block_file.read_block(0, &mut buf);
        assert_eq!(buf, buf1);

        get_block_cache(0, Arc::clone(&block_file) as Arc<dyn BlockDevice>)
            .lock()
            .read(0, |first_block: &[u8; 512]| assert_eq!(first_block, &buf1));

        get_block_cache(0, Arc::clone(&block_file) as Arc<dyn BlockDevice>)
            .lock()
            .modify(0, |first_block: &mut [u8; 512]| {
                first_block.copy_from_slice(&buf0);
            });

        get_block_cache(0, Arc::clone(&block_file) as Arc<dyn BlockDevice>)
            .lock()
            .read(0, |first_block: &[u8; 512]| assert_eq!(first_block, &buf0));
    }
}
