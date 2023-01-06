use std::{alloc, ptr};
use std::alloc::{alloc, Layout};
use std::ptr::NonNull;
use crate::lib::{initChunk };

mod lib;

fn main() {
    println!("Hello, world!");

    let a: [u8;0] = [];
    println!("Array is {:?}", a);

    let mut c = initChunk();
    println!("Initial array is {:?}", c);
    // swapChunk(Box::new([0u8, 1u8, 2u8]), &mut c);

    println!("Updated chunk : {:?}", c);

    // let a = Layout::array(5).unwrap();
    // alloc(a);

    let s = String::from("hello");
    let slice = &s[0..2];
    println!("Slice is >>{}<<", slice);

    let sentence = String::from("This is a long sentence");
    println!("First word of {} is {}", sentence, first_world(&sentence));

    let layout = Layout::array::<u8>(4).unwrap();
    let ptr = unsafe { alloc::alloc(layout) } ;
    // let ptr = NonNull::new(new_ptr as *mut u8).unwrap();

    unsafe {
        // ptr::write(ptr.as_ptr().add(0), 10u8);
        // ptr::write(ptr.as_ptr().add(1), 21u8);
        // ptr::write(ptr.as_ptr().add(2), 32u8);
        // ptr::write(ptr.as_ptr().add(3), 43u8);

        ptr::write(ptr.add(0), 10u8);
        ptr::write(ptr.add(1), 21u8);
        ptr::write(ptr.add(2), 32u8);
        ptr::write(ptr.add(3), 43u8);

        println!("Pointer to mem is {:?}", ptr::read(ptr.add(0)));
        let a = std::slice::from_raw_parts(ptr.add(1), 3);

        for i in a {
            println!("An elem is {:?}", i);
        }

        // let popped1 = ptr::read(ptr.add(3));
        // println!("Popped value :{:?}", popped1);

        // let new_layout = Layout::array::<u8>(1).unwrap();
        println!("First elem: {:?}, ptr: {:?}", ptr::read(ptr.add(0)), ptr);

        unsafe {
            alloc::dealloc(ptr, layout);
        }
        println!("Is ptr avail: {:?}", ptr);

        println!("Can you print same val from array: {:?}", ptr::read(ptr.add(0)));

    }


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