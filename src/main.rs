extern crate num;
#[macro_use]
extern crate num_derive;

use std::{ptr};
use crate::lib::{disassemble_chunk, init_chunk, write_chunk};
use crate::lib::OpCode::OpReturn;

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


    write_chunk(&mut c, OpReturn as u8);
    unsafe { println!("Output vec: {:?}", Vec::<u8>::from_raw_parts(c.code.as_ptr(), c.count, c.capacity)); }

    disassemble_chunk(&mut c, "test chunk");

}
