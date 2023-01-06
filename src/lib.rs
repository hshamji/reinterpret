use std::alloc::{self, Layout};
use std::ptr;
use std::cmp::max;
use std::ptr::NonNull;

pub enum OpCode {
    OP_RETURN,
}

#[derive(Debug)]
pub struct Chunk {
    code: NonNull<u8>,
    count: usize,
    capacity: usize,
}

pub fn initChunk() -> Chunk {
    Chunk {
        code: NonNull::<u8>::dangling(),
        count: 0,
        capacity: 0,
    }
}

pub fn writeChunk(c: &mut Chunk, val:u8) {
    if c.capacity < c.count+1 {
        let new_capacity = GROW_CAPACITY(c.capacity);
        // GROW_ARRAY(c, new_capacity);
    }

    unsafe { ptr::write(c.code.as_ptr().add(c.count), val);}
    c.count += 1;
}

fn GROW_CAPACITY(a: usize) -> usize {
    max(a * 2, 8)
}

fn GROW_ARRAY(c: &mut Chunk, new_capacity: usize) {
    if new_capacity == 0 {
        if c.count > 0 { // if there had been usage, deallocate, if not, do nothing
            unsafe {
                alloc::dealloc(c.code.as_ptr(), Layout::array::<u8>(c.capacity).unwrap());
            }
        }
        if c.count > 0 { // if it had contents, can deallocate, if not, then can't deallocate
        }
    } else {
    //     reallocate to a possibly new mem location
        unsafe {
            c.code = NonNull::new(alloc::realloc(c.code.as_ptr(), Layout::array::<u8>(c.capacity).unwrap(), new_capacity)).unwrap();
        }
    }

    c.capacity = new_capacity
}

fn main() {
    let a = Layout::array::<u8>(5).unwrap();
}
