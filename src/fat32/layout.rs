// from boot sector
#[repr(C)]
#[derive(Debug)]
pub struct BIOSParameterBlock {
    bootjump: [u8; 3],
    oem_name: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reversed_sector: u16,
    fat_number: u8,
    root_entry_count: u16,
    total_sectors_16: u16,
    media_type: u8,
    table_size_16: u16,
    sectors_per_track: u16,
    head_side_count: u16,
    hidden_sector_count: u32,
    total_sectors_32: u32,

    // Extended BIOS Paramter Block
    table_size_32: u32,
    extended_flags: u16,
    fat_version: u16,
    root_cluster: u32,
    fat_info: u16,
    backup_bs_sector: u16,
    reserved_0: [u8; 12],
    drive_number: u8,
    reserved_1: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    fat_type_label: [u8; 8],
}

pub struct FSInfo {
    // TODO
}
