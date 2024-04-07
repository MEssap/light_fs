pub enum OpenFlag {
    ReadOnly = 0x00,
    WriteOnly = 0x01,
    ReadWrite = 0x02,
    Create = 0x200,
    Trunc = 0x400,
}

pub struct File {}
