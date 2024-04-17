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
        easy_fs::{
            inode::{DiskInode, DiskInodeType},
            layout::SuperBlock,
            EasyFileSystem,
        },
        fat32::{layout::BIOSParameterBlock, FAT32FileSystem},
        BLOCK_CACHE_MANAGER,
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

    #[test]
    fn fat32_tests() {
        let block_file: Arc<BlockFile> = Arc::new(BlockFile(Mutex::new({
            let f = OpenOptions::new()
                .read(true)
                .write(true)
                .open("./fat32.img")
                .expect("cannot open img");
            f.set_len(100 * 1024 * 1024).unwrap();
            f
        })));

        let fat32_fs = FAT32FileSystem::new(block_file.clone());
        let root_dir = fat32_fs.root_directory();
        root_dir.ls();

        //let mut offset = 0;
        //get_block_cache(0, block_file.clone())
        //    .lock()
        //    .read(0, |block: &BIOSParameterBlock| {
        //        println!("{:#x?}", block);
        //        let table = block.get_table();
        //        offset = table.inner_block();
        //        //println!("{:#x?}", table);
        //        println!("{:#x?}", block.bytes_per_cluster());
        //    });

        //get_block_cache(offset, block_file.clone())
        //    .lock()
        //    .read(0, |block: &[u32; 32]| {
        //        println!("{:#x?}", block);
        //    });

        //let efs = EasyFileSystem::create(block_file.clone(), 100 * 1024 * 1024 / 512, 1);

        //let buf = [0x61u8; 32];

        //let root_inode = EasyFileSystem::root_inode(&efs);
        //root_inode.create("test.txt").unwrap().write_at(0, &buf);

        //BLOCK_CACHE_MANAGER.lock().sync_all();
    }
}
