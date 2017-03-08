//! Interrupts

// TODO auto-generate this file using `svd2rust`

use cortex_m::Reserved;

/// Interrupt handlers
pub struct Handlers {
    /// Reserved spot in the vector table
    pub reserved: [Reserved; 240],
}

/// Default interrupt handlers
pub const DEFAULT_HANDLERS: Handlers = Handlers {
    reserved: [Reserved::Vector; 240],
};
