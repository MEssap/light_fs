pub(crate) mod directory;
pub(crate) mod layout;
pub(crate) mod table;
pub mod vfs;

extern crate alloc;

use crate::{block::BlockDevice, cache::get_block_cache};
use alloc::sync::Arc;

use self::{directory::RootDirectory, layout::BIOSParameterBlock, table::FileAllocationTable};

pub struct FAT32FileSystem {
    pub block_device: Arc<dyn BlockDevice>,
    bytes_per_sector: usize,
    sectors_per_cluster: usize,
    table: FileAllocationTable,
}

impl FAT32FileSystem {
    pub fn new(block_device: Arc<dyn BlockDevice>) -> Self {
        let (bytes_per_sector, sectors_per_cluster, table) =
            get_block_cache(0, block_device.clone())
                .lock()
                .read(0, |bpb: &BIOSParameterBlock| {
                    (
                        bpb.bytes_per_sector(),
                        bpb.sectors_per_cluster(),
                        bpb.get_table(),
                    )
                });

        Self {
            block_device,
            bytes_per_sector: bytes_per_sector as usize,
            sectors_per_cluster: sectors_per_cluster as usize,
            table,
        }
    }

    pub fn is_valid(&self) -> bool {
        todo!()
    }

    pub fn root_directory(&self) -> RootDirectory {
        let start_cluster = get_block_cache(0, self.block_device.clone()).lock().read(
            0,
            |bpb: &BIOSParameterBlock| {
                (bpb.reversed_sectors_count() as usize
                    + bpb.hidden_sector_count() as usize
                    + bpb.fat_number() as usize * bpb.table_size_32() as usize)
                    / bpb.sectors_per_cluster() as usize
            },
        );

        RootDirectory::new(
            start_cluster,
            self.sectors_per_cluster,
            0,
            self.block_device.clone(),
        )
    }
}
