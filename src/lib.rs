use std::alloc::{self, Layout};
use std::cmp::max;
use std::ptr::NonNull;

pub enum OpCode {
    OP_RETURN,
}

#[derive(Debug)]
pub struct Chunk {
    code: Box<[u8]>,
    count: u32,
    capacity: u32,
}

pub fn initChunk() -> Chunk {
    Chunk {
        code: Box::new([]),
        count: 0,
        capacity: 0,
    }
}

pub fn swapChunk(mut a: Box<[u8]>, c: &mut Chunk) {
    c.code = a;
}

// pub fn writeChunk(c: &mut Chunk, val: u8) {
//     if c.capacity < c.count+1 {
//         let old_capacity = c.capacity;
//         c.capacity = GROW_CAPACITY(old_capacity);
//         c.code = GROW_ARRAY(val, &c.code, old_capacity, c.capacity)
//     }
// }

fn GROW_CAPACITY(a: u32) -> u32 {
    max(a * 2, 8)
}

fn main() {
    let a = Layout::array::<u8>(5).unwrap();
}
