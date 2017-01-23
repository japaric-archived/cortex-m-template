use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(_args: Arguments,
                               _file: &'static str,
                               _line: u32)
                               -> ! {
    hprint!("panicked at '");
    match () {
        #[cfg(feature = "semihosting")]
        () => {
            ::cortex_m_semihosting::io::write_fmt(_args);
        }
        #[cfg(not(feature = "semihosting"))]
        () => {}
    }
    hprintln!("', {}:{}", _file, _line);

    bkpt!();

    loop {}
}
