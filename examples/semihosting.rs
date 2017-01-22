#![no_main]
#![no_std]

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
