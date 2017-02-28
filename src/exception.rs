//! Exceptions

pub use cortex_m::exception::{DEFAULT_HANDLERS, Handlers, default_handler};

use cortex_m::asm;

/// The reset handler
///
/// This is the entry point of all programs
#[doc(hidden)]
#[export_name = "start"]
pub unsafe extern "C" fn reset_handler() -> ! {
    extern "C" {
        static mut _ebss: u32;
        static mut _sbss: u32;

        static mut _edata: u32;
        static mut _sdata: u32;

        static _sidata: u32;
    }

    ::r0::zero_bss(&mut _sbss, &mut _ebss);
    ::r0::init_data(&mut _sdata, &mut _edata, &_sidata);

    // NOTE `rustc` forces this signature on us. See `src/rt.rs`
    extern "C" {
        fn main(argc: isize, argv: *const *const u8) -> isize;
    }

    // Neither `argc` or `argv` make sense in bare metal contexts so we just
    // stub them
    main(0, ::core::ptr::null());

    // If `main` returns, then we go into "reactive" mode and attend interrupts
    // as they occur.
    loop {
        asm::wfi()
    }
}
