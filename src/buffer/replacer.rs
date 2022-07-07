pub trait Replacer {
    // replace return the next frame_id to be replaced. If no frames are eligible for replacement,
    // i.e. in cases like all frames are pinned, replace() return None
    fn replace() -> Option<u32>;

    // pin a frame_id, preventing it from turning up in the result of replace()
    // multiple calls to pin() is idempotent
    fn pin(frame_id: u32);

    // unpin a frame, unpinned frame are eligible in the replace() queue
    // multiple calls to unpin()
    fn unpin(frame_id: u32);

    // return the number of replacable frames
    fn size() -> usize;
}

pub struct LRUReplacer {}

impl Replacer for LRUReplacer {
    fn replace(&mut self) -> Option<u32>;

    fn pin(&mut self, frame_id: u32);

    fn unpin(&mut self, frame_id: u32);

    fn size(&self) -> usize;
}
