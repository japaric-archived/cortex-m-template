//! How to override the panic handler

#![no_std]

extern crate {{name}};

use {{name}}::{asm, exception, interrupt};

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
    asm::bkpt();

    loop {}
}

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
