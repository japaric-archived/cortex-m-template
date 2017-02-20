#![feature(collections)]
#![no_std]

#[macro_use]
extern crate {{name}};
#[macro_use]
extern crate collections;

use {{name}}::exceptions::{self, Exceptions};
use {{name}}::interrupts::{self, Interrupts};

fn main() {
    let xs = vec![0, 1, 2, 3];
    hprintln!("{:?}", xs);
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
