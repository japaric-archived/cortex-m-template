# `cortex-m-template`

> A Cargo project template to develop bare metal programs for Cortex-M
> microcontrollers.

# Table of contents

* [Features](#features)
* [How to use this template](#how-to-use-this-template)
  * [Step 1: Create a new Cargo project based on this template](#step-1-create-a-new-cargo-project-based-on-this-template)
  * [Step 2: Figure out the right "target" for your device](#step-2-figure-out-the-right-target-for-your-device)
      * [Example](#example)
  * [Step 3: Enter the memory specification of your device](#step-3-enter-the-memory-specification-of-your-device)
      * [Example](#example-1)
  * [Step 4: Build it](#step-4-build-it)
  * [Step 5: Flash and debug](#step-5-flash-and-debug)
* [Bonus How Tos](#bonus-how-tos)
  * [Debug an exception](#debug-an-exception)
  * [Override an exception handler](#override-an-exception-handler)
  * [Override the panic handler](#override-the-panic-handler)
  * [Auto-generate an API to access the peripherals](#auto-generate-an-api-to-access-the-peripherals)
  * [Use this Cargo project as a library](#use-this-cargo-project-as-a-library)
  * [Use interrupts](#use-interrupts)
  * [Semihosting](#semihosting)
* [Tips](#tips)
  - [`build.target`]#(buildtarget)
* [License](#license)
  * [Contribution](#contribution)

# Features

- With just a few steps, this Cargo project can be configured to work with *any*
  Cortex-M microcontroller.

- `static mut` variables work out of the box. The `.bss` and `.data` sections
  get initialized before you reach `main`.

- A `.gdbinit` file that automates the repetitive parts of your GDB sessions
  (connecting to OpenOCD, flashing, starting the program, etc.).

- All programs come with a catch-all exception handler (but which can be
  overridden) that gives you information about what triggered the exception.

- Logging / printing to the host's stdout via semihosting

- The panic handler can be overridden.

# How to use this template

## Step 1: Create a new Cargo project based on this template

```
$ cargo new myproject --template https://github.com/japaric/cortex-m-template

$ cd myproject
```

## Step 2: Figure out the right "target" for your device

This is the argument you pass to Cargo, via `--target`, to cross compile a Rust
program. For ARM Cortex-M microcontrollers, you have four choices:

- `thumbv6m-none-eabi`. For Cortex-M0, Cortex-M0+ and Cortex-M1
  microcontrollers.

- `thumbv7m-none-eabi`. For Cortex-M3 microcontrollers.

- `thumbv7em-none-eabi`. For Cortex-M4 and Cortex-M7 microcontrollers that
  **don't** have hardware support for floating point operations.

- `thumbv7em-none-eabihf`. For Cortex-M4F and Cortex-M7F microcontrollers that
  **do** have hardware support for floating point operations.

Pick one according to the characteristics of your device.

### Example

My target is the [STM32VLDISCOVERY][um] development board. This board has a
[STM32F100RBT6B][ds] microcontroller in it. This microcontroller is based on the
Cortex-M3 processor. Thus, I'll have to use the `thumbv7m-none-eabi` target.

[um]: http://www.st.com/resource/en/user_manual/cd00267113.pdf
[ds]: http://www.st.com/resource/en/datasheet/stm32f100v8.pdf

## Step 3: Enter the memory specification of your device

Edit the `MEMORY` section of the `memory.x` linker script:

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

The STM32F100RBT6B microcontroller has 128 KiB of Flash memory and 8 KiB of RAM.
I got this information from the [user manual of my development board][um]
(Section 2.1, page 7). The [data sheet of the microcontroller][ds] had this
information as well (Table 2, page 11).

With this information you can fill the `LENGTH` part of the linker script:

``` ld
MEMORY
{
  /* TODO You must correct these values */
  FLASH : ORIGIN = 0xBAAAAAAD, LENGTH = 128K
  RAM : ORIGIN = 0xBAAAAAAD, LENGTH = 8K
}
```

Then you have to enter at which addresses these two memory regions start. This
information seems harder to find. In my case, it was in
the [reference manual][rm] of the microcontroller (Section 2.3.1, page 39 and
Table 5, page 42-43). The Flash memory starts at `0x0800_0000` and the RAM
region starts at `0x2000_0000`:

[rm]: http://www.st.com/resource/en/reference_manual/cd00246267.pdf

``` ld
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 40K
}
```

## Step 4: Build it

You are done with the configuration. Now, you can build the starter app
in [`examples/app.rs`](examples/app.rs):

``` rust
//! Example application
//!
//! This shows all the parts that are required to build an application

// We don't link to the `std` crate because it's not available for Cortex-M
// devices.
#![no_std]

// We have to link our crate, obviously
extern crate {{name}};

// Instead of `std` we use the `core` crate, which provides the subset of
// `std`'s functionality that works on bare metal environments
use core::u32;

use {{name}}::exceptions::{self, Exceptions};
use {{name}}::interrupts::{self, Interrupts};

// We need a `main` function, just like every other Rust program
fn main() {
    let y: u32;
    let x: u32 = 0xDEADBEEF;
    y = u32::MAX;

    // You may be wondering what happens when `main` returns. The
    // microcontroller will go into a "reactive" mode where it services
    // interrupts as they occur and sleeps when there's nothing to do
}

// The program must specify how exceptions will be handled
// Here we just use the default handler to handle all the exceptions
#[no_mangle]
pub static _EXCEPTIONS: Exceptions =
    Exceptions { ..exceptions::DEFAULT_HANDLERS };

// Likewise with interrupts
#[no_mangle]
pub static _INTERRUPTS: Interrupts =
    Interrupts { ..interrupts::DEFAULT_HANDLERS };
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
subcommands work!) except that it will automatically build "standard" crates
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
 8000004:       08000401        .word   0x08000401

08000008 <_EXCEPTIONS>:
 8000008:       08000665 08000665 08000665 08000665     e...e...e...e...
 8000018:       08000665 00000000 00000000 00000000     e...............
 8000028:       00000000 08000665 00000000 00000000     ....e...........
 8000038:       08000665 08000665                       e...e...

08000040 <_INTERRUPTS>:
 8000040:       08000665 08000665 08000665 08000665     e...e...e...e...
        ...
 80003f0:       08000665 08000665 08000665 08000665     e...e...e...e...

08000400 <start>:
(..)

08000664 <myproject::exceptions::default_handler::h39bd3221f7330788>:
(..)
```

The actual values will probably be different in your case but you only have to
check these three lines:

- `08000000 <_VECTOR_TABLE>:`. The vector table must be at the beginning of the
  Flash memory region.

- ` 8000004:       08000401`. The RHS value must be `08000400 <start>` plus
  one.

- ` 8000008:       08000665 (..)`. The first value on the RHS must be `08000664
  <myproject::exceptions::default_handler::h39bd3221f7330788>` plus one.

### Step 5: Flash and debug

How to flash and debug the program may vary widely depending on the device but,
as an example, here are the steps to flash and debug a device supported by the
[OpenOCD] project:

[OpenOCD]: https://github.com/ntfreak/openocd

```
# Terminal 1. Leave this command running
$ openocd -f board/stm32vldiscovery.cfg
(..)
```

The arguments you have to pass to `openocd` will vary depending on your device
(and hardware programmer).

```
# Terminal 2
$ arm-none-eabi-gdb target/$TARGET/debug/examples/app
7       pub fn main() -> ! {
Breakpoint 1, myproject::exceptions::reset_handler () at $PWD/src/exceptions.rs:44
44      pub unsafe extern "C" fn reset_handler() -> ! {
(gdb)
```

Thanks to the `.gdbinit` file that's in the root of the Cargo project, this
command will flash your device, begin a debug session and start the execution of
your program. Afterwards, you can debug `app` like any other program.

![GDB session](https://i.imgur.com/4KQJJm2.png)
<p align="center">
<em>Interactive debug session using <a href="https://github.com/cyrus-and/gdb-dashboard">gdb-dashboard</a></em>
</p>

# Bonus How-Tos

## Debug an exception

[`examples/crash.rs`](examples/crash.rs) is an example that crashes.

When you execute this program under GDB, the program will first "hit" an
exception, then execute the default exception handler and finally stop execution
at the `bkpt!()` macro inside that handler. At that point, you can inspect the
previous stack frame through the local `_sf` variable.

```
Program received signal SIGTRAP, Trace/breakpoint trap.
myproject::exceptions::default_handler::handler (_sf=0x20001f54) at $PWD/src/exceptions.rs:22
22                  bkpt!();
(gdb) list
17          // stack frame
18          extern "C" fn handler(_sf: &StackFrame) -> ! {
19              hprintln!("EXCEPTION {:?} @ PC=0x{:08x}", Exception::current(), _sf.pc);
20
21              unsafe {
22                  bkpt!();
23              }
24
25              loop {}
26          }

(gdb) print/x *_sf
$1 = cortex_m::StackFrame {
  r0: 0x2fffffff,
  r1: 0x2fffffff,
  r2: 0x0,
  r3: 0x0,
  r12: 0x0,
  lr: 0x8000485,
  pc: 0x8000466,
  xpsr: 0x61000000
}
```

`_sf` holds a snapshot of the CPU registers at the moment the exception
occurred. And `_sf.pc`, in particular, points to the instruction that caused the
exception so you can "disassemble" your program around that address to see what
went wrong:
```
(gdb) print/x _sf.pc
$2 = 0x8000466

(gdb) disassemble/m _sf.pc
Dump of assembler code for function core::ptr::read_volatile<u32>:
299     pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x08000458 <+0>:     sub     sp, #20
   0x0800045a <+2>:     mov     r1, r0
   0x0800045c <+4>:     str     r0, [sp, #8]
   0x0800045e <+6>:     str     r1, [sp, #4]
   0x08000460 <+8>:     b.n     0x8000462 <core::ptr::read_volatile<u32>+10>
   0x08000462 <+10>:    ldr     r0, [sp, #8]
   0x08000464 <+12>:    str     r0, [sp, #12]

300         intrinsics::volatile_load(src)
   0x08000466 <+14>:    ldr     r0, [r0, #0]
   0x08000468 <+16>:    str     r0, [sp, #16]
   0x0800046a <+18>:    str     r0, [sp, #0]
   0x0800046c <+20>:    b.n     0x800046e <core::ptr::read_volatile<u32>+22>

301     }
   0x0800046e <+22>:    ldr     r0, [sp, #0]
   0x08000470 <+24>:    add     sp, #20
   0x08000472 <+26>:    bx      lr

End of assembler dump.
```

This line `0x08000466 <+14>:    ldr     r0, [r0, #0]` caused the exception; this
instruction tried to load the value at the address indicated by the register
`r0`. If you look at the fields of the `_sf` structure, you'll see `r0:
0x2fffffff`. This is the value that `r0` held when the exception was triggered.
Putting the two pieces together: What caused the exception was loading (`ldr`
instruction) the value at address `0x2fff_ffff` (`r0` argument). This makes
sense because the address `0x2fff_ffff` is well beyond the region of available
RAM on the device (`0x2000_0000 + 8K` in this case).

## Override an exception handler

[`examples/override-an-exception-handler.rs`](examples/override-an-exception-handler.rs) is
an example of overriding the default exception handler for a particular kind of
exception: the "hard fault" exception. Which is the exception that's raised when
the processor tries to access invalid memory.

If you run that program under GDB, you'll reach a `bkpt!()` statement just like
in the default exception handler case. But this time you'll be in the exception
handler declared in `override-an-exception-handler.rs`. Note that the `_sf`
argument is missing in this case.

```
Program received signal SIGTRAP, Trace/breakpoint trap.
override_an_exception_handler::custom_handler () at /home/japaric/tmp/myproject/examples/override-an-exception-handler.rs:31
31          bkpt!();
(gdb) list
26          ..exceptions::DEFAULT_HANDLERS
27      };
28
29      unsafe extern "C" fn custom_handler() {
30          // Once you hit the exception in `main`, you should reach this point!
31          bkpt!();
32      }
33
34      #[no_mangle]
35      pub static _INTERRUPTS: Interrupts =
```

The list of exceptions that can be overridden are in the `Exceptions` struct in
the [`src/exception.rs`](src/exception.rs) file.

## Override the panic handler

Each application can override the default panic handler.
Check
[`examples/override-the-panic-handler.rs`](examples/override-the-panic-handler.rs)
for details.

## Auto-generate an API to access the peripherals

Using peripherals involves reading the special memory regions indicated by the
[reference manual][rm]. Doing this directly is error prone; you may read/write
memory at the wrong address. Instead, you can use a code generator like
[svd2rust] to generate a type and memory safe API for you.

[svd2rust]: https://crates.io/crates/svd2rust

You'll need a System View Description (SVD) file for your microcontroller but
chances are that you'll find it in [this repository]. Then check svd2rust's
[documentation] to learn how to use the tool.

[this repository]: https://github.com/posborne/cmsis-svd/tree/master/data
[documentation]: https://github.com/docs.rs/svd2rust

## Use this Cargo project as a library

Apart from creating a new Cargo project and adding this crate as a dependency as
shown below:

```
$ cargo new --bin app && cd $_

# you can use path or git or version
$ edit Cargo.toml && tail -n2 $_
[dependencies.myproject]
path = "/path/to/the/myproject"
```

You'll have to add this template's `.cargo/config` file to the new Cargo
project:

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

But other than that, building your new binary Cargo project is no different than
building one of the examples in this Cargo project template:

```
$ xargo build --target $TARGET

$ arm-none-eabi-objdump -Cd target/$TARGET/debug/app

target/$TARGET/debug/app:     file format elf32-littlearm


Disassembly of section .text:

08000000 <_VECTOR_TABLE>:
 8000000:       2000a000        .word   0x2000a000
 8000004:       08000041        .word   0x08000041

08000008 <_EXCEPTIONS>:
 8000008:       08000665 08000665 08000665 08000665     e...e...e...e...
 8000018:       08000665 00000000 00000000 00000000     e...............
 8000028:       00000000 08000665 00000000 00000000     ....e...........
 8000038:       08000665 08000665                       e...e...

08000040 <_INTERRUPTS>:
 8000040:       08000665 08000665 08000665 08000665     e...e...e...e...
        ...
 80003f0:       08000665 08000665 08000665 08000665     e...e...e...e...

08000400 <start>:
(..)
```

## Use interrupts

If you want to use interrupts, you'll first have to "declare" how they are laid
out in the vector table. That's done with the `Interrupts` struct that resides
in `src/interrupts.rs`.

Before that, though, you must grab the documentation and search for a list of
interrupts available on your device. For example, for my microcontroller, that
information is in "Section 8.1.2 Interrupts and exceptions vectors" (page 130)
of the [reference manual][rm].

With that information, you'll have to populate the `Interrupts` struct. You must
make sure to preserve the order of the interrupt handlers and the reserved spots
in the vector table. Also, don't repeat the exception handlers which are already
in the `Exceptions` struct.

``` rust
// src/interrupt.rs

use cortex_m::Handler;
use exceptions::{self, Reserved};

/// Interrupt handlers
#[repr(C)]
pub struct Interrupts {
    /// Window Watchdog
    pub wwdg: Handler,

    /// PVD through EXTI Line16 detection
    pub pvd: Handler,

    ..

    /// ADC1 global interrupt
    pub adc1: Handler,

    /// Reserved spots in the vector table
    pub _reserved0: [Reserved; 4],

    /// EXTI Line[9:5] interrupts
    pub exti9_5: Handler,

    ..
}
```

Next, update the `DEFAULT_HANDLERS` constant. You can set all the handlers to
`exceptions::default_handler` or you could define a new default handler, only
for interrupts.

```
// src/interrupt.rs

pub const DEFAULT_HANDLERS: Interrupts = Interrupts {
    wwdg: exceptions::default_handler,
    pvd: exceptions::default_handler,
    ..
    adc1: exceptions::default_handler,
    ..
    _reserved: [Reserved::Vector; 4],
    exti9_5: exceptions::default_handler,
};
```

Now applications can override the interrupt handlers that they need:

``` rust
#[no_mangle]
pub static _INTERRUPTS: Interrupts = Interrupts {
    adc1: my_interrupt_handler,
    ..interrupts::DEFAULT_HANDLERS
};

unsafe extern "C" fn my_interrupt_handler() {
    ..
}
```

## Semihosting

If you enable the "semihosting" Cargo feature you'll be able to use semihosting
to write microcontroller programs that can send formatted messages to the host's
stdout, among other things. This semihosting feature is documented in
the [`cortex-m-semihosting` crate][sh].

[sh]: https://docs.rs/cortex-m-semihosting/0.1.0/cortex_m_semihosting/

# Tips

## `build.target`

You can set `build.target` in the `.cargo/config` to avoid having to pass
`--target $TARGET` every time you invoke Cargo.

``` diff
+[build]
+target = "thumbv7m-none-eabi"
+
 [target.thumbv6m-none-eabi]
 rustflags = [
   "-C", "link-arg=-Tmemory.x",
```

Then you can just call `xargo build --example app` to build an example.

**HEADS UP** This change will make `cargo install` unusable from within this
Cargo project. If you need to `cargo install` something, move to some parent
directory.

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
