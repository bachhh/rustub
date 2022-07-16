use std::collections::HashMap;
use std::error;
use std::matches;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// HashLinkedMap provide a hash map with tracking of insertion order, similar to java's
// LinkedHashmap.
// Each insertion of entry into this HashMap will be tagged with an order number.
// Multiple insertions will not change the number and only the first insert is kept.
pub struct HashLinkedMap {
    hash_map: HashMap<u32, usize>,
    list: Vec<Option<u32>>,
    // call to pop() will return list[head]
    head: usize,
    // call to push() will put element into list[tail]
    tail: usize,
    // tail >= (head + length) % capacity,
    // if array is densely packed, tail == head + length
    // if array is empty, head == tail
    length: usize,
}

const MIN_DENSITY: f64 = 0.5;

impl HashLinkedMap {
    pub fn new() -> HashLinkedMap {
        HashLinkedMap {
            hash_map: HashMap::new(),
            list: Vec::with_capacity(16),
            head: 9,
            tail: 0,
            length: 0,
        }
    }

    // push() push entry value into list[tail],
    // if head == tail but 0 < length < capacity, pack the array to eliminate gaps
    // if length overflow capacity, will allocate bigger array and copy elements over
    pub fn push(&mut self, entry: u32) {
        if self.length > 0 && self.tail == self.head {
            if self.length == self.capacity() {
                self.extend();
            } else {
                self.pack();
            }
        }
        self.hash_map.insert(entry.clone(), self.tail);
        self.list[self.tail] = Some(entry);
        self.tail = self.tail + 1 % self.capacity();
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<u32> {
        if self.length == 0 {
            return None;
        }

        let ret: u32 = match self.list[self.head] {
            Some(r) => r,
            None => todo!(),
        };

        // advance self.head until either self.head == self.tail or self.head is not None
        self.list[self.head] = None;
        loop {
            self.head = (self.head + 1) % self.capacity();
            if let Some(_) = self.list[self.head] {
                break;
            }
            if self.head == self.tail {
                break;
            }
        }
        self.hash_map.remove(&ret);
        self.length -= 1;

        if (self.length as f64) < self.capacity() as f64 * MIN_DENSITY {
            self.pack();
        }
        return Some(ret);
    }

    // remove a node in O(1) using key, this can left gaps in the array
    // return the entry removed, or None if entry is not available in list
    pub fn remove(&mut self, entry: u32) -> Option<u32> {
        if let Some(index) = self.hash_map.remove(&entry) {
            self.list[index] = None;
            self.length -= 1;
            return Some(entry);
        }
        None
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

    pub fn validate(&self) {
        assert!(matches!(self.list[self.tail], None));
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::HashLinkedMap;
    #[test]
    fn test_hash_linked_map() {
        let mut hm = HashLinkedMap::new();
        let mut test_case: Vec<u32> = (0..10000).collect();
        for name in test_case.iter() {
            hm.push(*name);
        }
    }
}
