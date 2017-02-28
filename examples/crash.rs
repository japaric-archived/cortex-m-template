//! This program crashes!
//!
//! The goal is to demonstrate what happens when the program hits an exception
//! and how the debug the problem.

#![no_std]

extern crate {{name}};

use core::ptr;

use {{name}}::{exception, interrupt};

fn main() {
    unsafe {
        ptr::read_volatile(0x2FFF_FFFF as *const u32);
    }
}

#[no_mangle]
pub static _EXCEPTIONS: exception::Handlers =
    exception::Handlers { ..exception::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: interrupt::Handlers =
    interrupt::Handlers { ..interrupt::DEFAULT_HANDLERS };
