//! The bootloader application
//! This application uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(asm)]
#![crate_type="staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate stellaris_launchpad_bootloader;
extern crate embedded_serial;

use stellaris_launchpad_bootloader::board;
use stellaris_launchpad_bootloader::cpu::uart;

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

use core::fmt::Write;
use embedded_serial::MutBlockingTx;

#[no_mangle]
pub extern "C" fn main() {
    let mut counter = 0;
    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);
    uart.puts("Welcome to TockOS Bootloader for the Stellaris Launchpad\n").unwrap();
    board::led_on(board::Led::Red);
    loop {
        writeln!(uart, "Hello {}!", counter).unwrap();
        if counter < 1024 {
            counter = counter + 1;          
        } else {
            counter = 0;
        }
        stellaris_launchpad_bootloader::delay(1000);
        board::led_on(board::Led::Blue);
        stellaris_launchpad_bootloader::delay(1000);
        board::led_off(board::Led::Blue);
    }
}

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
