//! What happens when `panic!` is invoked?
//!
//! Find out with this app

#![no_std]

extern crate {{name}};

use {{name}}::{exception, interrupt};

fn main() {
    panic!()
}

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
