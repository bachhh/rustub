# Rustub


A reimplementation of CMU Database course https://github.com/cmu-db/bustub in Rust



# Disk manager and Page file 

Page is a block of data of fixed size, defined in common header constant.
Each page are given a Page Number, starting from page 1. A disk manager
writes and read page to a single file on OS's file system. Pages are arrange
continuously in a file according to `OFFSET = PAGE_ID * PAGE_SIZE` where `OFFSET`
is the byte position on the file to start reading / writting.

