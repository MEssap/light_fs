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
        cache::{get_block_cache, BlockCache, BLOCK_SIZE},
        easy_fs::{bitmap::Bitmap, layout::SuperBlock, EasyFileSystem},
        BLOCK_CACHE_MANAGER,
    };
    use alloc::sync::Arc;
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom, Write},
    };
    use xx_mutex_lock::Mutex;

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

    #[test]
    fn mkfs() {
        let block_file = Arc::new(BlockFile(Mutex::new({
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open("./disk.img")
                .expect("cannot open img");
            f.set_len(100 * 1024 * 1024).unwrap();
            f
        })));

        let efs = EasyFileSystem::create(
            block_file as Arc<dyn BlockDevice>,
            100 * 1024 * 1024 / 512,
            1,
        );

        BLOCK_CACHE_MANAGER.lock().sync_all();
    }
}
