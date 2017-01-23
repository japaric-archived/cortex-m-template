#![no_main]
#![no_std]

#[cfg_attr(feature = "semihosting", macro_use)]
extern crate board;

#[no_mangle]
pub fn main() -> ! {
    match () {
        #[cfg(feature = "semihosting")]
        () => {
            hprintln!("Hello, world!")
        }
        #[cfg(not(feature = "semihosting"))]
        () => {}
    }

    loop {}
}
