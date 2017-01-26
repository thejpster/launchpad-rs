//! launchpad is crate for playing with the [Texas Instruments Stellaris
//! Launchpad](http://www.ti.com/tool/ek-lm4f120xl) (not to be confused with
//! the older MSP430 Launchpad). TI cancelled the Stellaris/LM4F range not
//! long after it came out and replaced it with Tiva-C/TM4C range. The [Tiva-C
//! Launchpad TM4C123G-XL](http://www.ti.com/tool/ek-tm4c123gxl) is almost
//! exactly the same as a Stellaris Launchpad and should be software
//! compatible. The Ethernet-enabled [TM4C1294 Connected
//! Launchpad](http://www.ti.com/tool/ek-tm4c1294xl) is not supported.
//!
//! It's very much a work in progress, but so far the UART, SysTick, Timer
//! (inc PWM) and GPIO seem to work. I'm gradually trying to follow the
//! example set by japaric in his [F3 crate](https://github.com/japaric/f3)
//! for the STM32F3 Discovery Board.

#![crate_type="staticlib"]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(start)]
#![feature(never_type)]
#![no_std]
#![warn(dead_code)]
#![deny(missing_docs)]

// ****************************************************************************
//
// Crates
//
// ****************************************************************************

extern crate compiler_builtins_snapshot;
#[macro_use]
extern crate cortex_m;
extern crate alloc_cortex_m;
pub extern crate lm4f120;
extern crate r0;
extern crate rlibc;
extern crate volatile_register;

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub mod board;
pub mod common;

pub use lm4f120 as cpu;

pub use cpu::systick::delay;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
