pub mod block;
pub mod cache;
pub mod ext4;
pub mod fat32;
pub mod file;

use crate::{block::BlockDevice, cache::BlockCacheManager};
use xx_mutex_lock::Mutex;

pub static BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> = Mutex::new(BlockCacheManager::new());

#[cfg(test)]
mod tests {
    extern crate alloc;
    extern crate std;

    use crate::{
        block::BlockDevice,
        cache::{self, get_block_cache, BlockCache, BlockCacheManager, BLOCK_SIZE},
        ext4::layout::SuperBlock,
        fat32::layout::BIOSParameterBlock,
        BLOCK_CACHE_MANAGER,
    };
    use alloc::sync::Arc;
    use std::{
        fs::{self, OpenOptions},
        io::{Read, Write},
    };
    use xx_mutex_lock::Mutex;

    static BLOCK_DEVICE: BlockDeviceImpl = BlockDeviceImpl;

    #[derive(Clone, Copy)]
    struct BlockDeviceImpl;
    impl BlockDevice for BlockDeviceImpl {
        fn read_block(&self, block_id: usize, buffer: &mut [u8]) {
            let mut block_file = OpenOptions::new().read(true).open("./disk.img").unwrap();
            block_file.read_exact(buffer).expect("read block wrong");
        }

        fn write_block(&self, block_id: usize, buffer: &[u8]) {
            let mut block_file = OpenOptions::new().read(true).open("./disk.img").unwrap();
            block_file.write_all(buffer).expect("write block wrong");
        }
    }

    #[test]
    fn cache_test() {
        let binding = get_block_cache(0, Arc::new(BLOCK_DEVICE));
        let cache = binding.lock();
        //let super_block = cache.get_ref::<BIOSParameterBlock>(0);
        let super_block = cache.get_ref::<SuperBlock>(0);
        println!("{:#x?}", super_block);
    }
}
