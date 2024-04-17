use std::ptr::NonNull;

// TODO:
// 根据获取目录中的目录项，访问对应的FAT表，并获取到文件

#[derive(Debug)]
pub struct FileAllocationTable {
    inner_offset: usize,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    table_size: u32,
}

impl FileAllocationTable {
    pub fn new(
        inner_offset: usize,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        table_size: u32,
    ) -> Self {
        Self {
            inner_offset,
            bytes_per_sector,
            sectors_per_cluster,
            table_size,
        }
    }

    pub fn entry(&self) {}

    pub fn inner_block(&self) -> usize {
        self.inner_offset / self.bytes_per_sector as usize
    }
}
