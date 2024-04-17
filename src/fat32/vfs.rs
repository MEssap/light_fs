// TODO:
// 虚拟文件系统层，封装文件操作(orwd)过程，需要
extern crate alloc;

use super::FAT32FileSystem;
use crate::block::BlockDevice;
use alloc::sync::Arc;
use xx_mutex_lock::Mutex;

pub struct FATFile {
    start_cluster: usize,
    sectors_per_cluster: usize,
    fs: Arc<Mutex<FAT32FileSystem>>,
    block_device: Arc<dyn BlockDevice>,
}
