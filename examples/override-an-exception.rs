#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
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

// NOTE the symbol name ("_hard_fault" in this case) must match the name of the
// "declaration" of the exception handler. (See the `extern "C"` block in the
// src/exception.rs file)
//
// The signature of this function must match that declaration as well. All the
// exception handlers must use the signature: `[unsafe] extern "C" fn()`.
#[export_name = "_hard_fault"]
pub unsafe extern "C" fn hard_fault_exception_handler() {
    bkpt!();
}
