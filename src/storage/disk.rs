use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
// use std::io::{BufReader, BufWriter};

use crate::storage::page::{Page, PageID, PAGE_SIZE};
use crate::utils::Result;

struct DiskManager {
    // file_name_: String,
    // num_flushes_: i32,
    // num_writes_: i32,
    // flush_log_: bool,
    // flush_log_f_: String, // TODO: pointer to WAL struct
    file: File,
}

// TODO: use custom error type instead of boxing
impl DiskManager {
    pub fn new(&self, db_path: &str) -> Result<DiskManager> {
        let file = File::options().read(true).write(true).open(db_path)?;
        Ok(DiskManager {
            // file_name_: db_path.to_string(),
            // num_flushes_: 0,
            // num_writes_: 0,
            // flush_log_: false,
            // flush_log_f_: "".to_string(),
            file: file,
        })
    }

    pub fn fetch_page(&mut self, page_id: u32) -> Result<Box<Page>> {
        let mut page = Box::new(Page {
            data: [0; PAGE_SIZE],
            is_dirty: false,
            pin_count: 0,
            page_id: PageID::ID(page_id),
        });

        self.file
            .seek(SeekFrom::Start((page_id as u64) * (PAGE_SIZE as u64)))?;
        self.file.read_exact(&mut page.data)?;
        // write
        Ok(page)
    }

    pub fn write_page(&mut self, page: &Box<Page>) -> Result<()> {
        match page.page_id {
            PageID::ID(pid) => {
                // TODO:
                self.file
                    .seek(SeekFrom::Start((pid as u64) * (PAGE_SIZE as u64)))?;
                self.file.write(&page.data)?;
                Ok(())
            }
            PageID::INVALID => Err("invalid page id")?,
        }
    }
}
