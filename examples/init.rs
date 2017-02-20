#![no_std]

#[macro_use]
extern crate {{name}};

use {{name}}::exceptions::{self, Exceptions};
use {{name}}::interrupts::{self, Interrupts};

pre_init_array!(before_before_main, {
    hprintln!("Hello, world!")
});

init_array!(before_main, {
    panic!("You've met with a terrible fate, haven't you?")
});

fn main() {
    unreachable!()
}

#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
