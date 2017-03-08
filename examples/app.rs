//! Example application
//!
//! This shows all the parts that are required to build an application

// We don't link to the `std` crate because it's not available for Cortex-M
// devices.
#![no_std]

// We have to link our crate, obviously
extern crate {{name}};

// Instead of `std` we use the `core` crate, which provides the subset of
// `std`'s functionality that works on bare metal environments
use core::u32;

use {{name}}::{exception, interrupt};

// We need a `main` function, just like every other Rust program
fn main() {
    let _y: u32;
    let _x: u32 = 0xDEADBEEF;
    _y = u32::MAX;
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
