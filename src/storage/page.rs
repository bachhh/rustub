use crate::utils::PageID;
use crate::utils::PAGE_SIZE;

pub struct Page {
    data: [u8; PAGE_SIZE],
    page_id: PageID,
    is_dirty: bool,
    pin_count: i8,
}
