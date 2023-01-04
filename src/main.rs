use std::{alloc, ptr};
use std::alloc::{alloc, Layout};
use crate::lib::{initChunk, swapChunk};

mod lib;

fn main() {
    println!("Hello, world!");

    let a: [u8;0] = [];
    println!("Array is {:?}", a);

    let mut c = initChunk();
    println!("Initial array is {:?}", c);
    swapChunk(Box::new([0u8, 1u8, 2u8]), &mut c);

    println!("Updated chunk : {:?}", c);

    // let a = Layout::array(5).unwrap();
    // alloc(a);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("Slice is >>{}<<", slice);

    let sentence = String::from("This is a long sentence");
    println!("First word of {} is {}", sentence, first_world(&sentence));

    let new_layout = Layout::array::<u8>(1).unwrap();
    let new_ptr = unsafe { alloc::alloc(new_layout) } ;
    unsafe { ptr::write()}

}

fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}