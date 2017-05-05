//! Based on `cortex-m-template`
//!
//! https://github.com/japaric/cortex-m-template

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(linkage)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc_cortex_m;
#[cfg(feature = "semihosting")]
#[macro_reexport(hprint, hprintln)]
#[macro_use]
extern crate cortex_m_semihosting;
extern crate compiler_builtins;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
#[macro_reexport(pre_init_array, init_array)]
#[cfg_attr(feature = "alloc", macro_use)]
extern crate r0;

#[macro_use]
mod macros;

mod lang_items;

pub mod exceptions;
pub mod interrupts;

#[cfg(feature = "alloc")]
init_array!(alloc, {
    extern "C" {
        static mut _edata: usize;
    }

    // 1KiB heap
    alloc_cortex_m::init(&mut _edata, (&mut _edata as *mut _).offset(256));
});
