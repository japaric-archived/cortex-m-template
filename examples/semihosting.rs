#![no_main]
#![no_std]

#[macro_use]
extern crate board;

#[no_mangle]
pub fn main() -> ! {
    hprintln!("Hello, world!");

    loop {}
}
