use std::error;
pub const PAGE_SIZE: usize = 4096;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum PageID {
    ID(u32),
    INVALID,
}
