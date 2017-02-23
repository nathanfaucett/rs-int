use core::ops::Neg;

use uint::UInt;


pub trait Int: UInt + Neg {}


impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for isize {}
