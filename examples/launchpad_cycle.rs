//! A rainbow-LED example application
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
use embedded_hal::serial::Read;
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
fn stellaris_main(mut board: stellaris_launchpad::board::Board) {
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
    let mut tr = tm4c123x_hal::pwm::Timer::timer0(&board.power_control, board.TIMER0)
        .into_odd(board.led_red.into_af_push_pull(&mut board.portf_control));
    let (mut tb, mut tg) = tm4c123x_hal::pwm::Timer::timer1(&board.power_control, board.TIMER1)
        .into_both(
            board.led_blue.into_af_push_pull(&mut board.portf_control),
            board.led_green.into_af_push_pull(&mut board.portf_control),
        );

    tr.set_period(255u32);
    tr.set_duty((), 0);
    tr.enable(());
    tb.set_period(255u32);
    tb.set_duty((), 0);
    tb.enable(());
    // Green is a bit bright! Tone it down.
    tg.set_period(512u32);
    tg.set_duty((), 0);
    tg.enable(());
    let mut angle = 0;
    loop {
        let (r, g, b) = calculate_rgb(angle);
        tr.set_duty((), r as u32);
        tb.set_duty((), b as u32);
        tg.set_duty((), g as u32);
        while let Ok(ch) = uart.read() {
            writeln!(uart, "byte read {}", ch).unwrap();
        }
        loops = loops + 1;
        angle = angle + 5;
        if angle >= 360 {
            angle -= 360;
            writeln!(uart, "Hello, world! Loops = {}", loops,).unwrap();
        };
        delay.delay_ms(50u32);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn calculate_rgb(angle: u16) -> (u8, u8, u8) {
    let angle = angle % 360;

    let g = match angle {
        x if x < 60 => ((x as f32) - 0.0) / 60.0,
        x if x < 180 => 1.0,
        x if x < 240 => 1.0 - (((x as f32) - 180.0) / 60.0),
        _ => 0.0,
    };

    let b = match angle {
        x if x < 120 => 0.0,
        x if x < 180 => ((x as f32) - 120.0) / 60.0,
        x if x < 300 => 1.0,
        x => 1.0 - (((x as f32) - 300.0) / 60.0),
    };

    let r = match angle {
        x if x < 60 => 1.0,
        x if x < 120 => 1.0 - (((x as f32) - 60.0) / 60.0),
        x if x < 240 => 0.0,
        x if x < 300 => ((x as f32) - 240.0) / 60.0,
        _ => 1.0,
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
