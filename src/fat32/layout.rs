extern crate alloc;

use super::table::FileAllocationTable;
use crate::{
    block::BlockDevice,
    cache::{get_block_cache, BlockCache},
};
use alloc::sync::Arc;
use core::ptr::NonNull;
use xx_mutex_lock::Mutex;

pub fn get_cluster_cache(
    start_cluster: usize,
    sectors_per_cluster: usize,
    block_device: Arc<dyn BlockDevice>,
) -> Arc<Mutex<BlockCache>> {
    get_block_cache(start_cluster / sectors_per_cluster, block_device)
}

// from boot sector
#[repr(C)]
#[derive(Debug)]
pub struct BIOSParameterBlock {
    bootjump: [u8; 3],               // 跳转指令，指向启动代码
    oem_name: [u8; 8], // 一段字符串域，很多情况下该域用于显示格式化该 FAT 卷的操作系统的名称
    bytes_per_sector: [u8; 2], // 每扇区字节数，取值只能是以下的几种情况：512、1024、2048或是4096
    sectors_per_cluster: u8, // 每簇扇区数，其值必须是2的整数次方
    reversed_sectors_count: [u8; 2], // 保留区中保留扇区的数目
    fat_number: u8,    // 此卷中FAT表的份数
    root_entry_count: [u8; 2], // FAT32，此项为 0
    total_sectors_16: [u8; 2], // FAT32，此项为 0
    media_type: u8, // 对于固定存储存储介质而言，0xF8是标准值，对于可移动存储介质，经常使用的数值是0xF0
    table_size_16: [u8; 2], // FAT32，此项为 0
    sectors_per_track: [u8; 2], // 每磁道扇区数
    head_side_number: [u8; 2], // 磁头数
    hidden_sector_count: [u8; 4], // 在此FAT分区之前所隐藏的扇区数，调用 BIOS 的0x13中断可以得到此数值
    total_sectors_32: [u8; 4],    // 该卷总扇区数

    // Extended BIOS Paramter Block
    table_size_32: [u8; 4],  // 一个FAT表所占的扇区数
    extended_flags: [u8; 2], // Bits 0-3:不小于 0 的活动 FAT（active FAT）数目，只有在镜像(mirroring)禁止时才有效。
    // Bits 4-6: 保留。
    // Bits 7: -- 0 表示 FAT 实时镜像到所有的 FAT 表中。-- 1 表示只有一个活动的 FAT 表，这个表就是bits 0-3 所指定的那个。
    // Bits 8-15: 保留
    fat_version: [u8; 2],       // 高位为 FAT32 的主版本号，底位为次版本号
    root_cluster: [u8; 4],      // 根目录所在第一个簇的簇号
    fat_info: [u8; 2],          // 保留区中FAT32卷FSINFO结构所占的扇区数，通常为1
    backup_bs_sectors: [u8; 2], // 在保留区中引导记录的备份数据所占的扇区数
    reserved_0: [u8; 12],       // 用于以后FAT的扩展使用
    drive_number: u8,           // 用于BIOS中断0x13得到磁盘驱动器参数
    reserved_1: u8,             // 保留
    boot_signature: u8,         // 扩展引导标记(0x29),用于指明此后的3个域可用
    volume_id: [u8; 4],         // 卷标序列号
    volume_label: [u8; 11],     // 磁盘卷标，没有卷标时，此域的内容为"NO NAME    "
    fat_type_label: [u8; 8],    // 以下的几种之一："FAT12   "、"FAT16   "、"FAT32   "
}

impl BIOSParameterBlock {
    pub fn bytes_per_sector(&self) -> u16 {
        (self.bytes_per_sector[0] as u16) + ((self.bytes_per_sector[1] as u16) << 8)
    }

    pub fn table_size_32(&self) -> u32 {
        (self.table_size_32[0] as u32)
            + ((self.table_size_32[1] as u32) << 8)
            + ((self.table_size_32[2] as u32) << 16)
            + ((self.table_size_32[3] as u32) << 24)
    }

    pub fn bytes_per_cluster(&self) -> usize {
        self.bytes_per_sector() as usize * self.sectors_per_cluster as usize
    }

    pub fn reversed_sectors_count(&self) -> u16 {
        (self.reversed_sectors_count[0] as u16) + ((self.reversed_sectors_count[1] as u16) << 8)
    }

    pub fn root_entry_count(&self) -> u16 {
        (self.root_entry_count[0] as u16) + ((self.root_entry_count[1] as u16) << 8)
    }

    pub fn root_cluster(&self) -> u32 {
        (self.root_cluster[0] as u32)
            + ((self.root_cluster[1] as u32) << 8)
            + ((self.root_cluster[2] as u32) << 16)
            + ((self.root_cluster[3] as u32) << 24)
    }

    pub fn hidden_sector_count(&self) -> u32 {
        (self.hidden_sector_count[0] as u32)
            + ((self.hidden_sector_count[1] as u32) << 8)
            + ((self.hidden_sector_count[2] as u32) << 16)
            + ((self.hidden_sector_count[3] as u32) << 24)
    }

    pub fn fat_number(&self) -> u8 {
        self.fat_number
    }

    pub fn sectors_per_cluster(&self) -> u8 {
        self.sectors_per_cluster
    }

    pub fn get_table(&self) -> FileAllocationTable {
        FileAllocationTable::new(
            self.reversed_sectors_count() as usize * self.bytes_per_sector() as usize,
            self.bytes_per_sector(),
            self.sectors_per_cluster,
            self.table_size_32(),
        )
    }
}

pub struct FSInfo {
    // TODO
}
