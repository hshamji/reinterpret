use std::{ptr};
use crate::lib::{init_chunk, write_chunk};

mod lib;

fn main() {
    let mut c = init_chunk();
    println!("Initial array is {:?}", c);

    write_chunk(&mut c, 10u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(0))});
    write_chunk(&mut c, 20u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(1))});
    write_chunk(&mut c, 30u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(2))});

}
