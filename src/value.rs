use std::alloc::{self, Layout};
use std::ptr;
use std::cmp::max;
use std::ptr::NonNull;




pub fn init_value_array() -> ValueArray {
    ValueArray {
        count: 0,
        capacity: 0,
        values: NonNull::<Value>::dangling(),
    }
}

pub fn write_value_array(c: &mut ValueArray, val: Value) {
    if c.capacity < c.count+1 {
        println!("Growing array");
        let new_capacity = grow_capacity(c.capacity);
        println!("New cap: {:?}", new_capacity);
        grow_array(c, new_capacity);
    }
    unsafe {
        ptr::write(c.values.as_ptr().add(c.count), val);
    }
    c.count += 1 ;
}

pub fn free_value_array(a: &mut ValueArray) {
    grow_array(a, 0);
}


fn grow_capacity(a: usize) -> usize {
    max(a * 2, 8)
}

fn grow_array(c: &mut ValueArray, new_capacity: usize) {
    if new_capacity == 0 {
        if c.count > 0 { // if there had been usage, deallocate, if not, do nothing
            unsafe {
                alloc::dealloc(c.values.as_ptr() as *mut u8, Layout::array::<Value>(c.capacity).unwrap());
            }
        }
        if c.count > 0 { // if it had contents, can deallocate, if not, then can't deallocate
        }
    } else if c.count == 0 {
        // first allocation
        c.values = NonNull::new(unsafe {
            alloc::alloc(Layout::array::<Value>(new_capacity).unwrap()) as *mut Value
        }
        ).unwrap(); // https://users.rust-lang.org/t/why-does-alloc-return-u8/55290/2
    }
    else {
        //     reallocate to a possibly new mem location
        unsafe {
            c.values = NonNull::new(alloc::realloc(c.values.as_ptr() as *mut u8, Layout::array::<u8>(c.capacity).unwrap(), new_capacity) as *mut Value).unwrap();
        }
    }

    c.capacity = new_capacity
}