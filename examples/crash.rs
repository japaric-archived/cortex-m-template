#![no_main]
#![no_std]

extern crate board;

use core::ptr;

#[no_mangle]
pub fn main() -> ! {
    // Read an invalid memory address
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }

    loop {}
}
