


use std::alloc::{self, Layout};
use std::ptr;
use std::cmp::max;
use std::ptr::NonNull;
use num_traits::FromPrimitive;

use crate::types::{Value, ValueArray, OpCode};
use crate::value::{init_value_array, write_value_array};



#[derive(Debug)]
pub struct Chunk {
    pub code: NonNull<u8>,
    pub count: usize,
    pub capacity: usize,
    pub constants: ValueArray,
}

pub fn init_chunk() -> Chunk {
    Chunk {
        code: NonNull::<u8>::dangling(),
        count: 0,
        capacity: 0,
        constants: init_value_array(),
    }
}

pub fn add_constant(c: &mut Chunk, v: Value) -> usize{
    write_value_array(&mut c.constants, v);
    c.constants.count - 1
}

pub fn write_chunk(c: &mut Chunk, val:u8) {
    if c.capacity < c.count+1 {
        println!("Growing array");
        let new_capacity = grow_capacity(c.capacity);
        println!("New cap: {:?}", new_capacity);
        grow_array(c, new_capacity);
    }
    unsafe {
        ptr::write(c.code.as_ptr().add(c.count), val);
    }
    c.count += 1 ;
}

pub fn free_chunk(c: &mut Chunk) {
    grow_array(c, 0);
}

pub fn disassemble_chunk(c: &mut Chunk, name: &str) {
    println!("== {} ==", name);
    unsafe {
        println!("Getting first few elems: {:#018b}", ptr::read(c.code.as_ptr().add(0)));
    }

    let mut i = 0 ;
    while i < c.count {
        i =  disassemble_instruction(c, i);
    }
}

// When need to start considering multiple bytes look at the `byteorder` crate https://stackoverflow.com/a/50244328
// Returns offset of the next instruction
fn disassemble_instruction(c: &mut Chunk, offset: usize) -> usize {
    print!("Offset: {} ", offset);

    unsafe {
        let instruction = ptr::read(c.code.as_ptr().add(offset));
        print!("Instruction: {}>Address:{:?}>", instruction, c.code.as_ptr());

        return match FromPrimitive::from_u8(instruction) {
            Some(OpCode::OpReturn) => {
                simple_instruction("OP_RETURN", offset)
            }
            _ => {
                println!("Unknown opcode: {:?}", instruction);
                offset + 1
            }
        };
    };
}

fn simple_instruction(name: &str, offset:usize) -> usize {
    println!("{name}");
    offset+1
}

fn grow_capacity(a: usize) -> usize {
    max(a * 2, 8)
}

fn grow_array(c: &mut Chunk, new_capacity: usize) {
    if new_capacity == 0 {
        if c.count > 0 { // if there had been usage, deallocate, if not, do nothing
            unsafe {
                alloc::dealloc(c.code.as_ptr(), Layout::array::<u8>(c.capacity).unwrap());
            }
        }
        if c.count > 0 { // if it had contents, can deallocate, if not, then can't deallocate
        }
    } else if c.count == 0 {
        // first allocation
        c.code = NonNull::new(unsafe {
            alloc::alloc(Layout::array::<u8>(new_capacity).unwrap())
        }
        ).unwrap();
    }
    else {
    //     reallocate to a possibly new mem location
        unsafe {
            c.code = NonNull::new(alloc::realloc(c.code.as_ptr(), Layout::array::<u8>(c.capacity).unwrap(), new_capacity)).unwrap();
        }
    }

    c.capacity = new_capacity
}

fn main() {}

#[test]
fn test_write_chunk_can_create_array() {
    let mut c = init_chunk();
    write_chunk(&mut c, 11u8);
    write_chunk(&mut c, 22u8);
    write_chunk(&mut c, 33u8);
    write_chunk(&mut c, 44u8);

    unsafe {
        let actual_vec = Vec::from_raw_parts(c.code.as_ptr(), c.count, c.capacity);
        let expected_vec: Vec::<u8> = vec![11,22,33,44];

        assert_eq!(actual_vec, expected_vec);
    };
}