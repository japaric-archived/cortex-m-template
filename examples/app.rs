#![no_main]
#![no_std]

extern crate board;

#[no_mangle]
pub fn main() -> ! {
    let y: u32;
    let x: u32 = 0xDEADBEEF;
    y = 0xBAAAAAAD;

    loop {}
}
