use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type Value = f64;
pub type Index = u16;

// https://stackoverflow.com/a/28029279 NOT https://enodev.fr/posts/rusticity-convert-an-integer-to-an-enum.html
// https://stackoverflow.com/questions/41648339/how-to-specify-the-underlying-type-of-an-enum-in-rust
#[derive(FromPrimitive)]
#[repr(u8)]
pub enum OpCode {
    OpReturn,
    OpConstant,
}

#[derive(Debug)]
pub struct ValueArray {
    pub capacity: usize,
    pub count: usize,
    pub values : NonNull<Value>,
}