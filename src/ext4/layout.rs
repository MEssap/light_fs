use super::def::EXT4FS_MAGIC;

#[repr(C)]
#[derive(Debug)]
pub struct SuperBlock {
    magic: u32,
}

//impl SuperBlock {
//    pub fn initialize(
//        &mut self,
//        total_blocks: u32,
//        inode_bitmap_blocks: u32,
//        inode_area_blocks: u32,
//        data_bitmap_blocks: u32,
//        data_area_blocks: u32,
//    ) {
//        *self = Self {
//            magic_number: EXT4FS_MAGIC,
//            total_blocks,
//            inode_bitmap_blocks,
//            inode_area_blocks,
//            data_bitmap_blocks,
//            data_area_blocks,
//        }
//    }
//
//    pub fn is_valid(&self) -> bool {
//        self.magic_number == EXT4FS_MAGIC
//    }
//}
