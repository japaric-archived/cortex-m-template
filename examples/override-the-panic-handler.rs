//! How to override the panic handler

#![feature(asm)]  // for `bkpt!`
#![no_std]

#[macro_use]  // for `bkpt!`
extern crate {{name}};

use {{name}}::{exception, interrupt};

fn main() {
    panic!()
}

// This is the new panic handler
// Make sure you get the function signature right!
#[no_mangle]
pub unsafe extern "C" fn rust_begin_unwind(_args: ::core::fmt::Arguments,
                                           _file: &'static str,
                                           _line: u32)
                                           -> ! {
    // After executing the `panic!` in `main`, you'll reach this breakpoint
    bkpt!();

    loop {}
}

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
