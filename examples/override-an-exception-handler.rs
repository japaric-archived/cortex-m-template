//! How to override an exception handler

#![no_std]

extern crate {{name}};

use core::ptr;

use {{name}}::{asm, exception, interrupt};

fn main() {
    // Read an invalid memory address. This triggers a "hard fault" exception
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers = exception::Handlers {
    // Here we override the default handler with a `custom_handler` but only
    // for hard fault exceptions.
    hard_fault: custom_handler,
    ..exception::DEFAULT_HANDLERS
};

unsafe extern "C" fn custom_handler<T>(_: &T) {
    // Once you hit the exception in `main`, you should reach this point!
    asm::bkpt();
}

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
