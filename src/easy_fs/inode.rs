extern crate alloc;

use crate::{
    block::BlockDevice,
    cache::{get_block_cache, BLOCK_SIZE},
};
use alloc::sync::Arc;

use super::layout::DataBlock;

const INODE_DIRECT_COUNT: usize = 28;
const INODE_INDIRECT1_COUNT: usize = BLOCK_SIZE / 4;
const DIRECT_BOUND: usize = 0;
const INDIRECT1_BOUND: usize = DIRECT_BOUND + INODE_INDIRECT1_COUNT;

type IndirectBlock = [u32; BLOCK_SIZE / 4];

#[derive(PartialEq)]
pub enum DiskInodeType {
    File,
    Directory,
}

#[repr(C)]
// 每个文件/目录在磁盘上均以一个DiskInode的形式存储，其中保存了文件的元数据
pub struct DiskInode {
    pub size: u32,                         // 文件大小/Bytes
    pub direct: [u32; INODE_DIRECT_COUNT], // 当文件很小的时候，使用直接索引，最多能保存INODE_DIRECT_COUNT个块
    pub indirect1: u32,                    // 一级索引
    pub indirect2: u32,                    // 二级索引
    // 使用查表方式的方式，根据索引号获取块设备号
    inode_type: DiskInodeType, // 索引类型
}

impl DiskInode {
    /// indirect1 and indirect2 block are allocated only when they are needed.
    pub fn initialize(&mut self, inode_type: DiskInodeType) {
        self.size = 0;
        self.direct.iter_mut().for_each(|v| *v = 0);
        self.indirect1 = 0;
        self.indirect2 = 0;
        self.inode_type = inode_type;
    }

    pub fn is_dir(&self) -> bool {
        self.inode_type == DiskInodeType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.inode_type == DiskInodeType::File
    }

    // 根据inode号，返回块设备号
    pub fn get_block_id(&self, inner_id: u32, block_device: &Arc<dyn BlockDevice>) -> u32 {
        let inner_id = inner_id as usize;

        if inner_id < INODE_DIRECT_COUNT {
            self.direct[inner_id]
        } else if inner_id < INDIRECT1_BOUND {
            get_block_cache(self.indirect1 as usize, Arc::clone(block_device))
                .lock()
                .read(0, |indirect_block: &IndirectBlock| {
                    indirect_block[inner_id - INODE_DIRECT_COUNT]
                })
        } else {
            let last = inner_id - INDIRECT1_BOUND;
            let indirect1 = get_block_cache(self.indirect2 as usize, Arc::clone(block_device))
                .lock()
                .read(0, |indirect2: &IndirectBlock| {
                    indirect2[last / INODE_INDIRECT1_COUNT]
                });

            get_block_cache(indirect1 as usize, Arc::clone(block_device))
                .lock()
                .read(0, |indirect1: &IndirectBlock| {
                    indirect1[last % INODE_INDIRECT1_COUNT]
                })
        }
    }

    fn _data_blocks(size: u32) -> u32 {
        // 向上取整
        (size + BLOCK_SIZE as u32 - 1) / BLOCK_SIZE as u32
    }

    /// Return block number correspond to size.
    pub fn data_blocks(&self) -> u32 {
        Self::_data_blocks(self.size)
    }

    /// Return number of blocks needed include indirect1/2.
    pub fn total_blocks(size: u32) -> u32 {
        let data_blocks = Self::_data_blocks(size) as usize;
        let mut total = data_blocks as usize;
        // indirect1
        if data_blocks > INODE_DIRECT_COUNT {
            total += 1;
        }
        // indirect2
        if data_blocks > INDIRECT1_BOUND {
            total += 1;
            // sub indirect1
            total +=
                (data_blocks - INDIRECT1_BOUND + INODE_INDIRECT1_COUNT - 1) / INODE_INDIRECT1_COUNT;
        }

        total as u32
    }

    pub fn blocks_num_needed(&self, new_size: u32) -> u32 {
        assert!(new_size >= self.size);
        Self::total_blocks(new_size) - Self::total_blocks(self.size)
    }

    // 扩充文件
    pub fn increase_size(
        &mut self,
        new_size: u32,
        new_blocks: Vec<u32>,
        block_device: &Arc<dyn BlockDevice>,
    ) {
        todo!()
    }

    /// Clear size to zero and return blocks that should be deallocated.
    ///
    /// We will clear the block contents to zero later.
    pub fn clear_size(&mut self, block_device: &Arc<dyn BlockDevice>) -> Vec<u32> {
        todo!()
    }

    /// 将文件内容从offset字节开始的部分读到内存中的缓冲区buf中
    /// 返回实际读到的字节数
    pub fn read_at(
        &self,
        offset: usize,
        buf: &mut [u8],
        block_device: &Arc<dyn BlockDevice>,
    ) -> usize {
        let mut start = offset;
        let end = (offset + buf.len()).min(self.size as usize);

        if start >= end {
            return 0;
        }

        let mut start_block = start / BLOCK_SIZE;
        let mut read_size = 0usize;

        loop {
            // calculate end of current block
            let mut end_current_block = (start / BLOCK_SIZE + 1) * BLOCK_SIZE;
            end_current_block = end_current_block.min(end);

            // read and update read size
            let block_read_size = end_current_block - start;
            let dst = &mut buf[read_size..read_size + block_read_size];

            get_block_cache(
                self.get_block_id(start_block as u32, block_device) as usize,
                Arc::clone(block_device),
            )
            .lock()
            .read(0, |data_block: &DataBlock| {
                let src = &data_block[start % BLOCK_SIZE..start % BLOCK_SIZE + block_read_size];

                dst.copy_from_slice(src);
            });
            read_size += block_read_size;

            // move to next block
            if end_current_block == end {
                break;
            }

            start_block += 1;
            start = end_current_block;
        }

        read_size
    }
}
