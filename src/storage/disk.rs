use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
// use std::io::{BufReader, BufWriter};

use crate::storage::page::{Page, PageID, PAGE_SIZE};
use crate::utils::Result;

// DiskManager abstract the disk manager process, reading and writting page data
// to and from the database file.
//
// DiskManager does not perform memory allocation for the Page struct itself.
pub struct DiskManager {
    // file_name_: String,
    // num_flushes_: i32,
    // num_writes_: i32,
    // flush_log_: bool,
    // flush_log_f_: String, // TODO: pointer to WAL struct
    file: File,
}

// TODO: use custom error type instead of boxing
impl DiskManager {
    pub fn new(db_path: &str) -> Result<DiskManager> {
        let file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(db_path)?;
        Ok(DiskManager {
            // file_name_: db_path.to_string(),
            // num_flushes_: 0,
            // num_writes_: 0,
            // flush_log_: false,
            // flush_log_f_: "".to_string(),
            file: file,
        })
    }

    // fetch_page read a page from the db file, writting the data into the pre-allocated Page object
    pub fn fetch_page(&mut self, page_id: u32, page: &mut Box<Page>) -> Result<()> {
        self.file
            .seek(SeekFrom::Start((page_id as u64) * (PAGE_SIZE as u64)))?;
        self.file.read_exact(&mut page.data)?;
        // write
        Ok(())
    }

    pub fn write_page(&mut self, page: &Box<Page>) -> Result<()> {
        match page.page_id {
            PageID::ID(pid) => {
                self.file
                    .seek(SeekFrom::Start((pid as u64) * (PAGE_SIZE as u64)))?;
                self.file.write(&page.data)?;
                Ok(())
            }
            PageID::INVALID => Err("invalid page id")?,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::DiskManager;
    use crate::storage::{Page, PageID, PAGE_SIZE};
    use std::fs;

    use rand::RngCore;

    #[test]
    fn disk_manager() {
        let db_file = "test.db";
        let mut disk = DiskManager::new(db_file).unwrap();
        // let's write some page datak
        let mut page1 = Box::new(Page {
            data: [0; PAGE_SIZE],
            is_dirty: false,
            pin_count: 0,
            page_id: PageID::ID(1),
        });

        let mut fake_data: [u8; 100] = [0; 100];
        rand::thread_rng().fill_bytes(&mut fake_data);
        for i in 0..100 {
            page1.data[i] = fake_data[i]; // copy scala
        }

        disk.write_page(&mut page1).unwrap();

        let mut page2 = Box::new(Page {
            data: [0; PAGE_SIZE],
            is_dirty: false,
            pin_count: 0,
            page_id: PageID::ID(1),
        });
        assert_ne!(page1.data, page2.data);
        disk.fetch_page(1, &mut page2).unwrap();
        assert_eq!(page1.data, page2.data);

        fs::remove_file(db_file).unwrap();
    }
}
