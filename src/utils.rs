use std::collections::HashMap;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// HashLinkedMap provide a hash map with tracking of insertion order, similar to java's
// LinkedHashmap.
// Each insertion of entry into this HashMap will be tagged with an order number.
// Multiple insertions will not change the number and only the first insert is kept.
pub struct HashLinkedMap {
    hash_map: HashMap<u32, usize>,
    list: Vec<Option<u32>>,
    head: usize,
    tail: usize,
    length: usize,
}

const MIN_DENSITY: f64 = 0.5;

impl HashLinkedMap {
    pub fn push(&mut self, entry: u32) {
        if self.length > 0 && self.tail == self.head {
            if self.length == self.capacity() {
                self.extend();
            } else {
                self.pack();
            }
        }
        self.tail = self.tail + 1 % self.capacity();
        self.hash_map.insert(entry.clone(), self.tail);
        self.list[self.tail] = Some(entry);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.length > 0 {
            let ret = if let Some(ret) = self.list[self.head] {
                ret
            } else {
                todo!()
            };
            self.list[self.head] = None;

            self.head = (self.head + 1) % self.capacity();
            while let None = self.list[self.head] {
                self.head = (self.head + 1) % self.capacity();
            }

            if (self.length as f64) < self.capacity() as f64 * MIN_DENSITY {
                self.pack();
            }
            self.hash_map.remove(&ret);
            self.length -= 1;
            return Some(ret);
        }
        None
    }

    pub fn remove(&mut self, entry: u32) {
        if let Some(index) = self.hash_map.remove(&entry) {
            self.list[index] = None;
            self.length -= 1;
        } else {
            todo!();
        }
    }

    fn capacity(&self) -> usize {
        return self.list.capacity();
    }

    // pack(), called when the density ( length / capacity ) went below certain level.
    // then the entire array is repacked by shifting all elements close together.
    //
    // pack() DOES NOT reduce capacity of the underlying Vector
    fn pack(&mut self) {
        let mut write = self.head;
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

    // 0..1..2...old_tail....head....old_cap.....new_cap.
    // Extend the array to [old_cap:new_cap], splice the section [0:old_tail] to
    // the new extended section, wrapping around if neccessary
    //
    // extend is called when push() causes length to exceed the current capacity.
    // It should be packed first if possible before calling extend.
    // After extend(), density (length/capacity) should be at least avg(min_density, max_density)
    fn extend(&mut self) {
        let old_capacity = self.capacity();
        self.list
            .reserve(((old_capacity as f64) / ((1.0 + MIN_DENSITY) * 2.0)) as usize);
        for i in old_capacity..=self.capacity() {
            self.list[i] = None;
        }

        if self.tail > self.head {
            return;
        }

        // tail parts are wrapped around, reinsert into the newe list tail
        let old_tail = self.tail;
        self.tail = old_capacity;
        for i in 0..old_tail {
            if let Some(entry) = self.list[i] {
                self.tail = self.tail + 1 % self.capacity();
                self.hash_map.insert(entry, self.tail);
                self.list.swap(self.tail, i); // list[i] is now None
            }
        }
    }
}
