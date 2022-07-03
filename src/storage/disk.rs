use std::error;
use std::fs::File;
// use std::io::{BufReader, BufWriter};
use std::result::Result;

use crate::storage::page::Page;
use crate::utils::PageID;

struct DiskManager {
    file_name_: String,
    num_flushes_: i32,
    num_writes_: i32,
    flush_log_: bool,
    flush_log_f_: String, // TODO: pointer to WAL struct
    file: File,
}

impl DiskManager {
    pub fn new(&self, db_path: &str) -> Result<DiskManager, Box<dyn error::Error>> {
        // TODO: use custom error type
        // std::scoped_lock scoped_db_io_latch(db_io_latch_);
        // db_io_.open(db_file, std::ios::binary | std::ios::in | std::ios::out);
        // // directory or file does not exist
        // if (!db_io_.is_open()) {
        //   db_io_.clear();
        //   // create a new file
        //   db_io_.open(db_file, std::ios::binary | std::ios::trunc | std::ios::out);
        //   db_io_.close();
        //   // reopen with original mode
        //   db_io_.open(db_file, std::ios::binary | std::ios::in | std::ios::out);
        //   if (!db_io_.is_open()) {
        //     throw Exception("can't open db file");
        //   }
        // }
        let file = File::options().read(true).write(true).open(db_path)?;
        Ok(DiskManager {
            file_name_: db_path.to_string(),
            num_flushes_: 0,
            num_writes_: 0,
            flush_log_: false,
            flush_log_f_: "".to_string(),
            file: file,
        })
    }

    // pub fn write_page(&self, page_id: PageID) -> Result<(), Box<dyn error::Error>> {}

    pub fn fetch_page(&self, page_id: PageID) -> Result<Page, Box<dyn error::Error>> {}
}
