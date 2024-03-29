pub(crate) mod bitmap;
pub(crate) mod def;
pub(crate) mod inode;
pub(crate) mod layout;

pub struct Ext4FileSystem {}

impl Ext4FileSystem {
    pub fn create() {}
    pub fn open() {}
    pub fn alloc_inode() {}
    pub fn alloc_data() {}
    pub fn dealloc_data() {}
}
