use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_args: Arguments,
                               _file: &'static str,
                               _line: u32)
                               -> ! {
    match () {
        #[cfg(feature = "semihosting")]
        () => {
            hprint!("panicked at '");
            ::cortex_m_semihosting::io::_write_fmt(_args);
            hprintln!("', {}:{}", _file, _line);
        }
        #[cfg(not(feature = "semihosting"))]
        () => {}
    }

    bkpt!();

    loop {}
}
