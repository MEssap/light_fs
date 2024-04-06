use core::{mem::size_of, str::from_utf8};

const NAME_LENGTH_LIMIT: usize = 27;
pub const DIRENT_SIZE: usize = size_of::<DirectoryEntry>();

#[repr(C)]
pub struct DirectoryEntry {
    name: [u8; NAME_LENGTH_LIMIT + 1],
    inode_number: u32,
}

impl DirectoryEntry {
    pub fn new(name: &str, inode_number: u32) -> Self {
        let mut bytes = [0u8; NAME_LENGTH_LIMIT + 1];
        let _ = &mut bytes[..name.len()].copy_from_slice(name.as_bytes());
        Self {
            name: bytes,
            inode_number,
        }
    }

    pub fn empty() -> Self {
        Self {
            name: [0u8; NAME_LENGTH_LIMIT + 1],
            inode_number: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *const u8, DIRENT_SIZE) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as usize as *mut u8, DIRENT_SIZE) }
    }

    pub fn name(&self) -> &str {
        let len = (0usize..).find(|i| self.name[*i] == 0).unwrap();
        from_utf8(&self.name[..len]).unwrap()
    }

    pub fn inode_number(&self) -> u32 {
        self.inode_number
    }
}
