use std::collections::HashMap;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// HashLinkedMap provide a hash map with tracking of insertion order, similar to java's
// LinkedHashmap.
// Each insertion of entry into this HashMap will be tagged with an order number.
// Multiple insertions will not change the number and only the first insert is kept.
pub struct HashLinkedMap<T> {
    hash_map: HashMap<T, u32>,
    list: Vec<Option<T>>,
    head: usize,
    tail: usize,
    length: usize,
}

const MIN_DENSITY: f64 = 0.5;

impl<T> HashLinkedMap<T> {
    pub fn push(&mut self, entry: T) {
        if self.length > 0 && self.tail == self.head {
            if self.length == self.capacity() {
                self.extend();
            } else {
                self.pack();
            }
        }
        self.list[self.tail] = Some(entry);
        self.tail = self.tail + 1 % self.capacity();
        self.length += 1;
    }

    fn capacity(&self) -> usize {
        return self.list.capacity();
    }

    // pack is called when the density ( length / capacity ) went below certain level.
    // then the entire array is repacked by shifting all elements close together.
    fn pack(&mut self) {
        let mut write = self.head.clone();
        let mut read = write;
        let mut count = 0;
        while count < self.length {
            if let None = self.list[write] {
                // advance read until we see a none empty slot
                if read <= write {
                    read = (write + 1) % self.capacity();
                }
                while let None = self.list[read] {
                    read = (read + 1) % self.capacity();
                }

                self.list.swap(read, write);
            }
            write = (write + 1) % self.capacity();
            count += 1;
        }
    }

    // extend is called when push() causes length to exceed the current capacity.
    // If the array is packable, it should be packed first before considering for expansion.
    // after resize, density (length/capacity) should be at least avg(min_density, max_density)
    fn extend(&mut self) {
        let old_capacity = self.capacity();
        self.list
            .reserve(((old_capacity as f64) / ((1.0 + MIN_DENSITY) * 2.0)) as usize);
        // splice the "front" of array to the tail of array

        let old_tail = self.tail;
        self.tail = old_capacity;
        for i in 0..old_tail {
            if let Some(entry) = self.list[i] {
                self.push(entry);
                self.list[i] = None;
            }
        }
    }
}
