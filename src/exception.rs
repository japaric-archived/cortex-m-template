//! Exceptions

use cortex_m::{Handler, StackFrame};

// This default exception handler gives you access to the previous stack frame
// which is where the exception occurred. To be able to do that, the handler is
// split in two parts: `_default_exception_handler` and `deh`.
//
// NOTE Do NOT modify this function. You are free to modify the `deh` function
// below though.
#[doc(hidden)]
#[naked]
#[no_mangle]
pub unsafe extern "C" fn _default_exception_handler() {
    use core::intrinsics;

    // NOTE need asm!, #[naked] and unreachable() to avoid modifying the stack
    // pointer (MSP) so it points to the previous stack frame
    asm!("mrs r0, MSP
          ldr r1, [r0, #20]
          b $0" :: "i"(deh as extern "C" fn(&StackFrame) -> !) :: "volatile");

    intrinsics::unreachable();
}

// Default exception handler that has access to previous stack frame `_sf`
extern "C" fn deh(_sf: &StackFrame) -> ! {
    unsafe {
        bkpt!();
    }

    loop {}
}

// Reset handler
#[doc(hidden)]
#[export_name = "_reset"]
pub unsafe extern "C" fn reset() -> ! {
    extern "C" {
        static _ebss: u32;
        static _edata: u32;
        static _sidata: u32;
        static mut _sbss: u32;
        static mut _sdata: u32;
    }

    if &_sbss as *const _ as usize != &_ebss as *const _ as usize {
        ::r0::zero_bss(&mut _sbss, &_ebss);
    }

    if &_sdata as *const _ as usize != &_edata as *const _ as usize {
        ::r0::init_data(&mut _sdata, &_edata, &_sidata);
    }

    ::init();

    extern "Rust" {
        // `main`, the entry point of the user program
        // NOTE the right signature of `main` is `fn() -> !`. But the user might
        // get that wrong so let's err on the side of caution and install a
        // safety net. (See below)
        fn main();
    }

    main();

    // safety net in case `main` returns
    panic!("returned from `main`!")
}

// List of all the exceptions minus the reset handler as allocated in the
// vector table.
//
// `None` indicates that the spot is RESERVED.
#[doc(hidden)]
#[export_name = "_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<Handler>; 14] = [Some(_nmi),
                                                Some(_hard_fault),
                                                Some(_memmanage_fault),
                                                Some(_bus_fault),
                                                Some(_usage_fault),
                                                None,
                                                None,
                                                None,
                                                None,
                                                Some(_svcall),
                                                None,
                                                None,
                                                Some(_pendsv),
                                                Some(_systick)];

extern "C" {
    /// Non-maskable interrupt.
    pub fn _nmi();

    /// All class of fault.
    pub fn _hard_fault();

    /// Memory management.
    pub fn _memmanage_fault();

    /// Pre-fetch fault, memory access fault.
    pub fn _bus_fault();

    /// Undefined instruction or illegal state.
    pub fn _usage_fault();

    /// System service call via SWI instruction
    pub fn _svcall();

    /// Pendable request for system service
    pub fn _pendsv();

    /// System tick timer
    pub fn _systick();
}
