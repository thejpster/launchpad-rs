//! A blinky-LED example application
//! This example uses launchpad-rs.

#![no_std]
#![no_main]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate embedded_hal;
extern crate stellaris_launchpad;
extern crate tm4c123x_hal;

use core::fmt::Write;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::serial::Read as ReadHal;
use embedded_hal::Pwm;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::serial;
use tm4c123x_hal::time::Bps;

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
#[no_mangle]
pub fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
    let mut pins_a = board.GPIO_PORTA.split(&board.power_control);
    let mut uart = serial::Serial::uart0(
        board.UART0,
        pins_a.pa1.into_af_push_pull(&mut pins_a.control),
        pins_a.pa0.into_af_push_pull(&mut pins_a.control),
        (),
        (),
        Bps(115200),
        serial::NewlineMode::SwapLFtoCRLF,
        stellaris_launchpad::board::clocks(),
        &board.power_control,
    );
    let mut delay = tm4c123x_hal::delay::Delay::new(
        board.core_peripherals.SYST,
        stellaris_launchpad::board::clocks(),
    );
    let mut loops = 0;

    let mut blue_led_pwm = tm4c123x_hal::pwm::Timer::timer1(&board.power_control, board.TIMER1)
        .into_even(board.led_blue.into_af_push_pull(&mut board.portf_control));

    blue_led_pwm.set_period(4096u32);
    blue_led_pwm.set_duty((), 0);
    blue_led_pwm.enable(());

    let levels = [1u32, 256, 512, 1024, 2048, 4096];
    uart.write_all("Welcome to Launchpad Blink\n");
    loop {
        for level in &levels {
            blue_led_pwm.set_duty((), *level);
            writeln!(uart, "Hello, world! Loops = {}, level = {}", loops, level).unwrap();
            while let Ok(ch) = uart.read() {
                writeln!(uart, "byte read {}", ch).unwrap();
            }
            loops = loops + 1;
            delay.delay_ms(250u32);
        }
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
