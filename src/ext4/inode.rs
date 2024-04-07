use super::def::INODE_DIRECT_COUNT;
use crate::cache::BLOCK_SIZE;

type IndirectBlock = [u32; BLOCK_SIZE / 4];

#[derive(PartialEq)]
pub enum InodeType {
    File,
    Directory,
}

pub struct Inode {
    pub size: u32,
    pub direct: [u32; INODE_DIRECT_COUNT],
    pub indirect1: u32,
    pub indirect2: u32,
    type_: InodeType,
}

impl Inode {
    pub fn new() {}
    pub fn find() {}
    pub fn create() {}
    pub fn read_at() {}
    pub fn write_at() {}

    pub fn initialize(&mut self, type_: InodeType) {
        self.size = 0;
        self.direct.iter_mut().for_each(|v| *v = 0);
        self.indirect1 = 0;
        self.indirect2 = 0;
        self.type_ = type_;
    }

    pub fn is_dir(&self) -> bool {
        self.type_ == InodeType::Directory
    }

    pub fn is_file(&self) -> bool {
        self.type_ == InodeType::File
    }
}
