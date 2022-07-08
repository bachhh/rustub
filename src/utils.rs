use std::collections::HashMap;
use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

// HashLinkedMap provide a hash map with tracking of insertion order, similar to java's
// LinkedHashmap.
// Each insertion of entry into this HashMap will be tagged with an order number.
// Multiple insertions will not change the number and only the first insert is kept.
pub struct HashLinkedMap<T> {
    hash_map: HashMap<T, u32>,
    list: Vec<T>,
    head: u32,
    tail: u32,
    length: u32,
    capacity: u32,
}
impl<T> HashLinkedMap<T> {
    pub fn push(&mut self, entry: T) {}

    // pack is called when the density ( length / capacity ) went below certain level.
    // then the entire array is repacked by shifting all elements close together.
    fn pack(&mut self) {}

    // expand is called when push() causes length to exceed the current capacity.
    // If the array is packable, it should be packed first before considering for expansion.
    fn expand(&mut self) {}
}
