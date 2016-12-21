//! A crate to hack the $DEVELOPMENT_BOARD!

#![feature(asm)]
#![feature(compiler_builtins_lib)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

extern crate compiler_builtins;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
extern crate r0;

mod lang_items;

pub mod exception;
pub mod interrupt;

// "Pre `main`" initialization routine
fn init() {}
