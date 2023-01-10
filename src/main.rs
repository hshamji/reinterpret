use std::{alloc, ptr};
use std::alloc::{alloc, Layout};
use std::ptr::{NonNull, write};
use crate::lib::{initChunk, writeChunk };

mod lib;

fn main() {
    let mut c = initChunk();
    println!("Initial array is {:?}", c);

    writeChunk(&mut c, 10u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(0))});
    writeChunk(&mut c, 20u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(1))});
    writeChunk(&mut c, 30u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(2))});

}
