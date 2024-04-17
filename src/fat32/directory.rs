extern crate alloc;

use crate::block::BlockDevice;
use alloc::sync::Arc;

use super::layout::get_cluster_cache;

// TODO:
// 访问根目录，能够以长文件名的方式保存和获取文件的对应条目

pub struct RootDirectory {
    start_cluster: usize,
    sectors_per_cluster: usize,
    size: usize,
    pub block_device: Arc<dyn BlockDevice>,
}

pub type FileName = [u8; 128];

impl RootDirectory {
    pub fn new(
        start_cluster: usize,
        sectors_per_cluster: usize,
        size: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Self {
        Self {
            start_cluster,
            sectors_per_cluster,
            size,
            block_device,
        }
    }

    pub fn is_long(&self) -> bool {
        // TODO
        true
    }

    pub fn ls(&self) -> Result<Vec<String>, ()> {
        if self.is_long() {
            let mut files: Vec<String> = vec![];

            get_cluster_cache(
                self.start_cluster,
                self.sectors_per_cluster,
                self.block_device.clone(),
            )
            .lock()
            .read(0, |filename: &FileName| {
                let mut file: Vec<char> = vec![];

                for i in filename {
                    file.push(*i as char);
                }

                println!("{:#x?}", file);
            });

            Ok(files)
        } else {
            Err(())
        }
    }
}
