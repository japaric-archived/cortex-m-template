# `cortex-m-template`

> A minimal Cargo project template to develop Rust programs for Cortex-M
> microcontrollers.

**NOTE** This template is *aimed* at users who are familiar with building C/C++
or Rust programs for microcontrollers from scratch. However, I expect that most
people will be able to make this work even if they don't understand some of the
concepts/terminology used here. If you want to know why this works and why it's
written/organized the way it is, you can read the [Copper] book.

[Copper]: https://japaric.github.com/copper

# Table of contents

* [Table of contents](#table-of-contents)
* [Features](#features)
* [Usage](#usage)
  * [Step 1: Figure out the right "target" for your device](#step-1-figure-out-the-right-target-for-your-device)
      * [Example](#example)
  * [Step 2: Enter the memory specification of your device](#step-2-enter-the-memory-specification-of-your-device)
      * [Example](#example-1)
  * [Step 3: There's no step 3](#step-3-theres-no-step-3)
* [Bonus HowTos](#bonus-howtos)
  * [Inspect the previous stack when an exception occurs](#inspect-the-previous-stack-when-an-exception-occurs)
  * [Override an exception handler](#override-an-exception-handler)
  * [Auto-generate an API to access the peripherals](#auto-generate-an-api-to-access-the-peripherals)
  * [Use this Cargo project as a library](#use-this-cargo-project-as-a-library)
  * ["Install" interrupts](#install-interrupts)
* [License](#license)
  * [Contribution](#contribution)

# Features

- With just two steps, this Cargo project can be configured to work with any
  Cortex-M microcontroller.

- `static mut` variables work out of the box. The `.bss` and `.data` sections
  get initialized before you reach `main`.

- A `.gdbinit` file that automates the repetitive parts of your GDB sessions
  (connecting to OpenOCD, flashing, etc.) and takes you directly to the `main`
  function.

- All programs come with a catch-all exception handler (but which can be
  overridden) that gives you information about what triggered the exception.

# Usage

Or how to adjust this template so it will work for your microcontroller.

## Step 1: Figure out the right "target" for your device

This is the argument you pass to Cargo, via `--target`, to cross compile a Rust
program for your microcontroller. You have four choices:

- `thumbv6m-none-eabi`. For Cortex-M0, Cortex-M0+ and Cortex-M1
  microcontrollers.

- `thumbv7m-none-eabi`. For Cortex-M3 microcontrollers.

- `thumbv7em-none-eabi`. For Cortex-M4 and Cortex-M7 microcontrollers that
  **don't** have hardware support for floating point operations.

- `thumbv7em-none-eabihf`. For Cortex-M4F and Cortex-M7F microcontrollers that
  **do** have hardware support for floating point operations.

Pick one according to the characteristics of your device.

### Example

My target is the [STM32F3DISCOVERY][um] development board. This board has a
[STM32F303VCT6][ds] microcontroller in it. This microcontroller is based on a
Cortex-M4F processor that has hardware support for floating point operations.
Thus, I'll have to use the `thumbv7em-none-eabihf` target.

[um]: http://www.st.com/resource/en/user_manual/dm00063382.pdf
[ds]: http://www.st.com/resource/en/datasheet/stm32f303vc.pdf

## Step 2: Enter the memory specification of your device

Edit the first part of the `memory.x` linker script:

``` ld
MEMORY
{
  /* TODO You must correct these values */
  FLASH : ORIGIN = 0xBAAAAAAD, LENGTH = 0
  RAM : ORIGIN = 0xBAAAAAAD, LENGTH = 0
}
```

to match the memory layout of your device.

### Example

The STM32F303VCT6 microcontroller has 256 KiB of Flash memory and 40 KiB of RAM.
I got this information from the [user manual of my development board][um]
(Section 1, page 6). The [data sheet of the microcontroller][ds] had this
information as well (Table 2, page 11).

With this information you can fill the `LENGTH` part of the linker script:

``` ld
MEMORY
{
  /* TODO You must correct these values */
  FLASH : ORIGIN = 0xBAAAAAAD, LENGTH = 256K
  RAM : ORIGIN = 0xBAAAAAAD, LENGTH = 40K
}
```

Then you have to enter at which addresses these two memory regions start. This
information seems harder to find. In my case, it was in
the [reference manual][rm] of the microcontroller (Table 2, page 53-54). The
Flash memory starts at `0x0800_0000` and the RAM region starts at `0x2000_0000`:

[rm]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

``` ld
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

## Step 3: There's no step 3

You are done with the configuration. Now, you can build the default example
that's in [`examples/app.rs`](examples/app.rs):

``` rust
#![no_main]
#![no_std]

extern crate board;

#[no_mangle]
pub fn main() -> ! {
    let y: u32;
    let x: u32 = 0xDEADBEEF;
    y = 0xBAAAAAAD;

    loop {}
}
```

You'll need to use Xargo instead of Cargo, so install it first:

```
$ cargo install xargo
```

Xargo depends on the `rust-src` component so install that as well:

```
$ rustup component add rust-src
```

Xargo works exactly like Cargo (the subcommands are the same, even custom
subcommands work!) except that it will automatically build and "link" crates
like `core` for you:

```
# Use the target you chose in the step 1
$ xargo build --target $TARGET --example app
```

Your executable will be in `target/$TARGET/debug/examples`. It's a good a idea
to confirm that the [vector table][vt] is where it's supposed to be and that's
properly initialized:

[vt]: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0553a/BABIFJFG.html

```
$ arm-none-eabi-objdump -Cd target/$TARGET/debug/examples/app
Disassembly of section .text:

08000000 <_VECTOR_TABLE>:
 8000000:       2000a000        .word   0x2000a000
 8000004:       08000041        .word   0x08000041

08000008 <_EXCEPTIONS>:
 8000008:       080001df 080001df 080001df 080001df     ................
 8000018:       080001df 00000000 00000000 00000000     ................
 8000028:       00000000 080001df 00000000 00000000     ................
 8000038:       080001df 080001df                       ........

08000040 <_reset>:

(..)

080001de <_bus_fault>:
```

The actual values will probably be different in your case but you only have to
check these two lines:

- ` 8000004:       08000041`. The RHS value must be `08000040 <_reset>` plus
  one.

- ` 8000008:       080001df (..)`. The first value on the RHS must be `080001de
  <_bus_fault>` plus one.

Then you can flash and debug your program.

How to flash and debug the program will vary widely depending on the device but,
as an example, here are the steps to flash and debug a device supported by the
[OpenOCD] project:

[OpenOCD]: https://github.com/ntfreak/openocd

```
# Terminal 1. Leave this command running
$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
(..)
```

The arguments you have to pass to `openocd` will vary depending on your device
(and hardware programmer).

```
# Terminal 2
$ arm-none-eabi-gdb -q target/$TARGET/debug/examples/app
Breakpoint 1, app::main ()
    at $PWD/examples/app.rs:7
7       pub fn main() -> ! {
(gdb)
```

Thanks to the `.gdbinit` file that's in the root of the Cargo project, this will
flash your device and start a debug section dropping you right at the start of
the `main` function. From where you can interactively debug your program using
e.g. `layout src`:

![GDB TUI](https://i.imgur.com/lbSkBxO.png)

# Bonus HowTos

## Inspect the previous stack when an exception occurs

[`examples/crash.rs`](examples/crash.rs) is an example that crashes because it
tries to read an invalid memory address:

If you execute this program under GDB, the program will "hit" an exception, then
execute the default exception handler (a handler is just a function) and then
stop execution at the `bkpt!()` macro inside that handler. The `bkpt!` macro,
which is just [the bkpt instruction][bkpt], yields control back to GDB. At that
point, you can inspect the previous stack frame via the local `_sf` variable.

[bkpt]: https://developer.arm.com/docs/dui0553/latest/3-the-cortex-m4-instruction-set/312-miscellaneous-instructions/3121-bkpt

```
Program received signal SIGTRAP, Trace/breakpoint trap.
    at $PWD/src/exception.rs:34
34          bkpt!();
(gdb) list
29
30      // Default exception handler that has access to previous stack frame `_sf`
31      #[doc(hidden)]
32      #[export_name = "_default_exception_handler_impl"]
33      pub unsafe extern "C" fn default_handler(_sf: &StackFrame) -> ! {
34          bkpt!();
35
36          loop {}
37      }
38

(gdb) print/x *_sf
$1 = cortex_m::StackFrame {
  r0: 0x2fffffff,
  r1: 0x2fffffff,
  r2: 0x0,
  r3: 0x0,
  r12: 0x0,
  lr: 0x80000f5,
  pc: 0x80000d6,
  xpsr: 0x61000200
}
```

`_sf` holds a snapshot of the CPU registers at the moment the exception
occurred. And `_sf.pc`, in particular, points to the instruction that caused the
exception so you can "disassemble" your program around that address to see what
went wrong:
```
(gdb) print/x _sf.pc
$1 = 0x80000d6

(gdb) disassemble/m _sf.pc
Dump of assembler code for function core::ptr::read_volatile<u32>:
213     pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x080000c8 <+0>:     sub     sp, #20
   0x080000ca <+2>:     mov     r1, r0
   0x080000cc <+4>:     str     r0, [sp, #16]
   0x080000ce <+6>:     str     r1, [sp, #4]
   0x080000d0 <+8>:     b.n     0x80000d2 <core::ptr::read_volatile<u32>+10>
   0x080000d2 <+10>:    ldr     r0, [sp, #16]
   0x080000d4 <+12>:    str     r0, [sp, #12]
   0x080000d6 <+14>:    ldr     r0, [r0, #0]
   0x080000d8 <+16>:    str     r0, [sp, #8]

214         intrinsics::volatile_load(src)
   0x080000da <+18>:    str     r0, [sp, #0]
   0x080000dc <+20>:    b.n     0x80000de <core::ptr::read_volatile<u32>+22>

215     }
   0x080000de <+22>:    ldr     r0, [sp, #0]
   0x080000e0 <+24>:    add     sp, #20
   0x080000e2 <+26>:    bx      lr

End of assembler dump.
```

This line `0x080000d6 <+14>:    ldr     r0, [r0, #0]` caused the exception; it
tries to load the value at the address that the register `r0` indicates. If you
look at the fields of the `_sf` structure, you'll see `r0: 0x2fffffff`. This
indicates that `r0` held that value right when the exception was triggered.
Putting the two pieces together: What caused the exception was loading (`ldr`
instruction) the value at address `0x2fff_ffff` (`r0` argument).

## Override an exception handler

[`examples/override-an-exception.rs`](examples/override-an-exception.rs) is an
example of overriding the default exception handler for a particular exception:
the "hard fault" exception. Which is the exception that accessing invalid memory
raises.

If you run that program under GDB, just like the default exception handler case,
you'll reach a `bkpt!()` statement. But this time you'll be in the exception
handler declared in *that* program. Note that the `_sf` argument is missing as
well.

```
Program received signal SIGTRAP, Trace/breakpoint trap.
override_an_exception::hard_fault_exception_handler ()
    at $PWD/examples/override-an-exception.rs:22
22          bkpt!();
(gdb) list
17          loop {}
18      }
19
20      #[export_name = "_hard_fault"]
21      pub unsafe extern "C" fn hard_fault_exception_handler() {
22          bkpt!();
23      }
```

The list of exceptions that can be overridden are in an `extern "C"` block in
the [`src/exception.rs`](src/exception.rs) file.

## Auto-generate an API to access the peripherals

Using peripherals involves reading special memory regions indicated by the
[reference manual][rm]. Doing this directly is error prone; you may read/write
memory at the wrong address. Instead, you can use a code generator like
[svd2rust] to generate a memory safe API for you.

[svd2rust]: https://crates.io/crates/svd2rust

You'll need a System View Description (SVD) file for your microcontroller
though. Chances are you'll find it in [this repository].

[this repository]: https://github.com/posborne/cmsis-svd/tree/master/data

Once you got the right file, you can get the base addresses of your
microcontroller peripherals with the following commands:

```
# install svd2rust first
$ cargo install svd2rust

$ mkdir src/peripheral

# NOTE adjust the name of the SVD file accordingly
$ svd2rust -i STM32F30x.svd > src/peripheral/mod.rs

$ head -n5 src/peripheral/mod.rs
const GPIOA: usize = 0x48000000;
const GPIOB: usize = 0x48000400;
const GPIOC: usize = 0x48000800;
const GPIOD: usize = 0x48000c00;
const GPIOE: usize = 0x48001000;
```

To actually generate the API code for a peripheral, use these commands:

```
# "rcc" is the name of the peripheral
$ svd2rust -i $SVD_FILE rcc > src/peripheral/rcc.rs
```

The [volatile-register][vr] crate plus some extra glue code is needed:

[vr]: https://crates.io/crates/volatile-register

```
$ cargo add volatile-register
```

``` diff
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -12,11 +12,13 @@
 #[macro_use]
 extern crate cortex_m;
 extern crate r0;
+extern crate volatile_register;

 mod lang_items;

 pub mod exception;
 pub mod interrupt;
+pub mod peripheral;

 // "Pre `main`" initialization routine
 fn init() {}
```

``` diff
--- src/peripheral/mod.rs
+++ src/peripheral/mod.rs
@@ -1,3 +1,19 @@
+#![allow(dead_code)]
+#![allow(non_upper_case_globals)]
+
+pub mod rcc;
+
+use self::rcc::Rcc;
+
+pub fn rcc() -> &'static Rcc {
+    unsafe { &*(RCC as *const _) }
+}
+
+// NOTE(unsafe) hands over mutable references to the same memory region
+pub unsafe fn rcc_mut() -> &'static mut Rcc {
+    &mut *(RCC as *mut _)
+}
+
 const GPIOA: usize = 0x48000000;
 const GPIOB: usize = 0x48000400;
 const GPIOC: usize = 0x48000800;
```

Then you'll be able to write applications like this:

``` rust
#![no_main]
#![no_std]

extern crate board;

use board::peripheral;

#[no_mangle]
pub fn main() -> ! {
    let rcc = unsafe { peripheral::rcc_mut() };

    // Set the IOPAEN bit of the AHBENR register, which is in the RCC register
    // block, to 1
    rcc.ahbenr.modify(|_, w| w.iopaen(true));

    loop {}
}

```

The API generated by svd2rust is documented
[here](https://github.com/japaric/svd2rust#api).

## Use this Cargo project as a library

Apart from creating a new Cargo project and adding this crate as a dependency as
shown below:

```
$ cargo new --bin app && cd $_

# use path or git or version
# adjust the name of the dependency if you have renamed the project template
$ tail -n2 Cargo.toml
[dependencies.board]
path = "/path/to/the/configured/template"
```

You'll have to add this template `.cargo/config` file to the new Cargo project:

```
$ mkdir -p .cargo

$ curl -LSfs \
  https://raw.githubusercontent.com/japaric/cortex-m-template/master/.cargo/config \
  > .cargo/config
```

Then you can use `example/app.rs` as a starter code for your new project:

```
$ curl -LSfs \
  https://raw.githubusercontent.com/japaric/cortex-m-template/master/examples/app.rs \
  > src/main.rs
```

But other than that, building your new binary Cargo project is no different from
building an example in this Cargo project template:

```
$ xargo build --target $TARGET

$ arm-none-eabi-objdump -Cd target/$TARGET/debug/app

target/$TARGET/debug/app:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_VECTOR_TABLE>:
 8000000:       0200a000        .word   0x0200a000
 8000004:       08000041        .word   0x08000041

08000008 <_EXCEPTIONS>:
 8000008:       0800025f 0800025f 0800025f 0800025f     _..._..._..._...
 8000018:       0800025f 00000000 00000000 00000000     _...............
 8000028:       00000000 0800025f 00000000 00000000     ...._...........
 8000038:       0800025f 0800025f                       _..._...
```

## "Install" interrupts

If you want to use interrupts, you'll first have to "install" them in the vector
table. The `board` crate is already structured for this kind of addition.

First of all, you must grab the documentation and search for a list of
interrupts available in your device. For example, that information is in
"Section 14.1 Nested vectored interrupt controller (NVIC)" (page 286) of
the [reference manual][rm] of my microcontroller.

Next, you'll have to "declare" all the available interrupt handlers in an
`extern "C"` block in the [`src/interrupt.rs`](src/interrupts.rs) source file:

> **NOTE** The exception handlers have already been declared in
> `src/exception.rs`; you don't need to repeat them in `src/interrupt.rs`.

``` rust
// src/interrupt.rs

// See table 81 of the STM32F3DISCOVERY reference manual.
extern "C" {
    /// Window Watchdog
    pub fn _wwdg();

    /// PVD through EXTI Line16 detection
    pub fn _pvd();

    // ..
}
```

Then, you have to place these **in order** in the vector table. To do that fill
the `_INTERRUPTS` static variable in the `src/interrupt.rs` file with the
functions you just declared. If your manual indicates that some spot in the
vector table is "reserved", use `None` for the corresponding element of the
array, otherwise use `Some(_interrupt_handler)`.

``` rust
// src/interrupt.rs

#[doc(hidden)]
#[export_name = "_INTERRUPTS"]
pub static _INTERRUPTS: [Option<Handler>; 85] = [
    Some(_wwdg),
    Some(_pvd),
    (..)
    Some(_fmc)
    None,
    None,
    Some(_spi3)
    (..)
];
```

Next, you have to provide a default implementation for the interrupt handlers.
The simplest thing to do is "weakly" bind them to the
`_default_exception_handler` symbol which is the default exception handler. You
can do that in the `memory.x` linker script:

``` ld
/* Interrupts */
PROVIDE(_wwdg = _default_exception_handler);
PROVIDE(_pvd = _default_exception_handler);
(..)
```

To use the interrupts you have to override the corresponding handler just like
you would do with an exception handler:

``` rust
#[no_mangle]
pub fn main() -> ! {
    // main loop
}

#[export_name = "_tim7"]
pub extern "C" fn basic_timer_isr() {
    // Interrupt Service Routine
}
```

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
