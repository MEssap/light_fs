const BLOCK_SIZE: usize = 0x400;

pub const EXT4FS_MAGIC: u32 = 0;

pub const INODE_DIRECT_COUNT: usize = 28;
pub const INODE_INDIRECT1_COUNT: usize = BLOCK_SIZE / 4;
//pub const INDIRECT1_BOUND: usize = DIRECT_BOUND + INODE_INDIRECT1_COUNT;
