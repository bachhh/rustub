pub const PAGE_SIZE: usize = 4096;

pub enum PageID {
    ID(u32),
    INVALID,
}

pub struct Page {
    pub data: [u8; PAGE_SIZE],

    // page_id of the page, start from 0
    pub page_id: PageID,

    pub is_dirty: bool,
    pub pin_count: i8,
}
