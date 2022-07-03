# Rustub

A reimplementation of CMU Database course https://github.com/cmu-db/bustub in Rust

# Disk manager and Page file 

Page is a block of data of fixed size, defined in common header constant.
Each page are given a Page Number, starting from page 1. A disk manager
writes and read page to a single file on OS's file system. Pages are arrange
continuously in a file according to `OFFSET = PAGE_ID * PAGE_SIZE` where `OFFSET`
is the byte position on the file to start reading / writting.

## Disk Manager

## Page 

A Page is a strictly aligned struct data for storing arbitary data structure.
Page form the basis of the storage engine to writting to and from a file. A
Page struct starts with 4096 bytes of binary data right at head, following by
various variables used for Page operation:

- page_id: keep track of page_id.
- pin_count: used by CachePolicy, track how many threads is accessing Page.
- is_dirty: used by DiskManager, tracks if Page need flushing to disk.
- rw_latch: base level concurrency control.

```
0          4096             
+-----------+---------+----------+----------+----------+
|  DATA     | page_id | pin_count| is_dirty | rw_latch |
+-----------+---------+----------+----------+----------+
```

