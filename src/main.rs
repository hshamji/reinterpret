// extern crate num;
// #[macro_use]
// extern crate num_derive;

use std::{ptr};
use crate::chunk::{disassemble_chunk, init_chunk, write_chunk, add_constant};
use crate::chunk::OpCode::{OpReturn, OpConstant};
use crate::value::{init_value_array, write_value_array};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

mod chunk;
mod value;


fn main() {
    let mut c = init_chunk();
    println!("Initial array is {:?}", c);

    write_chunk(&mut c, 11u8);
    println!("Size: {}, Value at offset: {:#018b}", c.count, unsafe{ptr::read(c.code.as_ptr().add(0usize))});
    write_chunk(&mut c, 22u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(1usize))});
    write_chunk(&mut c, 33u8);
    println!("Size: {}, Value at offset: {:?}", c.count, unsafe{ptr::read(c.code.as_ptr().add(2usize))});
    println!("Address of array: {:?}", c.code.as_ptr());

    write_chunk(&mut c, OpReturn as u8);
    unsafe { println!("Output vec: {:?}", Vec::<u8>::from_raw_parts(c.code.as_ptr(), c.count, c.capacity)); }

    disassemble_chunk(&mut c, "test chunk");

    // let mut t = &[13u8,5u8];
    // println!("1:{:b}, 2:{:b}", t[0],t[1]);
    //
    // let t2 = t.read_u16::<BigEndian>();
    // println!("t2 res: {:?}", t2);

    let mut data = [1, 16, 1, 2];
    let mut current = &data[..];
    println!("Current {:?}", current);

    let v1 = current.read_u16::<LittleEndian>();
    let v2 = current.read_u16::<BigEndian>();
    println!("{:?}, {:?}", v1, v2); // Ok(4097), Ok(258)

    // let d = [1,16];
    // let d2 = data[..2];
    // let val = u16::from_be_bytes(d2);
    // println!("data: {:?}, converted: {:?}", data[1..3], val);

    let loc = add_constant(&mut c, 1.2);
    write_chunk(&mut c, OpConstant as u8);
    write_chunk(&mut c, loc as u8);

}
