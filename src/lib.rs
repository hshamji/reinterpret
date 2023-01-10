use std::alloc::{self, Layout};
use std::ptr;
use std::cmp::max;
use std::ptr::NonNull;

extern crate num;
#[macro_use]
extern crate num_derive;

// https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
// https://stackoverflow.com/questions/41648339/how-to-specify-the-underlying-type-of-an-enum-in-rust
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    OpReturn=0,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: NonNull<u8>,
    pub count: usize,
    pub capacity: usize,
}

pub fn init_chunk() -> Chunk {
    Chunk {
        code: NonNull::<u8>::dangling(),
        count: 0,
        capacity: 0,
    }
}

pub fn write_chunk(c: &mut Chunk, val:u8) {
    if c.capacity < c.count+1 {
        println!("Growing array");
        let new_capacity = grow_capacity(c.capacity);
        println!("New cap: {}", new_capacity);
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

    let mut i = 0 ;
    while i < c.count {
        i =  disassemble_instruction(c, i);
    }
}

// Returns offset of the next instruction
fn disassemble_instruction(c: &mut Chunk, offset: usize) -> usize {
    print!("{:?} ", offset);

    unsafe {
        let instruction = ptr::read(c.code.as_ptr().add(offset));

        let op_code = num::FromPrimitive::from_u8(instruction).expect(&format!("Unknown Opcode: {:?}", instruction));

        match op_code {
            OpCode::OpReturn => {
                return simpleInstruction("OP_RETURN", offset)
            }
            _ => {
                println!("Unknown opcode: {:?}", instruction);
                return offset+1
            }
        };
    };
}

fn simpleInstruction(name: &str, offset:usize) -> usize {
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