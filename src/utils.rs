pub const PAGE_SIZE: usize = 4096;

pub const INVALID_PAGE_ID: i32 = -1;

pub enum PageID {
    ID(i32),
    INVALID,
}
