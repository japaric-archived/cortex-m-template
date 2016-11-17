//! Interrupts

use cortex_m::Handler;

extern "C" {
    // TODO declare the interrupt handlers here. Example below:
    //
    // fn _wwdg();
    // fn _pvd();
}

// List of the interrupts as allocated in the vector table.
#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static _INTERRUPTS: [Option<Handler>; 0] = [
    // TODO then order those interrupt handler as they appear in the
    // documentation. Use `None` where the spot in the vector table is marked as
    // RESERVED. Example below:
    //
    // Some(_wwdg),
    // Some(_pvd),
    // (..)
    // Some(_fmc)
    // None,
    // None,
    // Some(_spi3)
    // (..)
];
