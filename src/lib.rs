#![no_std]

pub mod block;
pub mod cache;
pub mod easy_fs;
// pub mod ext4;
// pub mod fat32;

use crate::cache::BlockCacheManager;
use spin::Mutex;

pub static BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> = Mutex::new(BlockCacheManager::new());

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use crate::{
        block::BlockDevice,
        cache::{get_block_cache, BLOCK_SIZE},
        easy_fs::{
            inode::{DiskInode, DiskInodeType},
            layout::SuperBlock,
            EasyFileSystem,
        },
        BLOCK_CACHE_MANAGER,
    };
    use alloc::sync::Arc;
    use spin::Mutex;
    use std::{
        fs::{File, OpenOptions},
        io::{Read, Seek, SeekFrom, Write},
    };

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
                .create(false)
                .truncate(true)
                .open("./efs.img")
                .expect("cannot open img");
            f.set_len(100 * 1024 * 1024).unwrap();
            f
        })));

        let efs = EasyFileSystem::create(block_file.clone(), 100 * 1024 * 1024 / 512, 1);
        let efs = EasyFileSystem::open(block_file.clone());

        let buf = [0x61u8; 32];

        let root_inode = EasyFileSystem::root_inode(&efs);
        root_inode.create("test.txt").unwrap().write_at(0, &buf);

        BLOCK_CACHE_MANAGER.lock().sync_all();
    }
}
