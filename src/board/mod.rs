//! A board support library for the TI Stellaris Launchpad

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

use lm4f120::{fpu, pll, systick};
pub use lm4f120::gpio;
use embedded_hal::digital::OutputPin;

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

#[derive(PartialEq, Clone, Copy)]
/// The Launchpad has a tri-colour LED, which we consider
/// to be three separate LEDs.
pub enum Led {
    /// The Red LED
    Red,
    /// The Blue LED
    Blue,
    /// The Green LED
    Green,
}

#[derive(PartialEq, Clone, Copy)]
/// The Launchpad has two buttons
pub enum Button {
    /// SW1
    One,
    /// SW2
    Two,
}

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

/// The pin used for the Red LED
pub const LED_RED: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin1);
/// The pin used for the Blue LED
pub const LED_BLUE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin2);
/// The pin used for the Green LED
pub const LED_GREEN: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin3);
/// The pin used for Button One
pub const BUTTON_ONE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin0);
/// The pin used for Button Two
pub const BUTTON_TWO: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin4);

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Initialise everything on the board - FPU, PLL, SysTick, GPIO and the LEDs
/// and buttons. Should be pretty much the first call you make in `main()`.
/// Doesn't init the UART - that's separate.
pub fn init() {
    fpu::init();
    pll::init(pll::ClockSpeed::Speed66MHz);
    systick::init();
    gpio::init();
    enable_buttons();
    enable_leds();
}

/// Turn an LED on
pub fn led_on(led: Led) {
    match led {
        Led::Red => LED_RED.set_high(),
        Led::Blue => LED_BLUE.set_high(),
        Led::Green => LED_GREEN.set_high(),
    }
}

/// Turn an LED off
pub fn led_off(led: Led) {
    match led {
        Led::Red => LED_RED.set_low(),
        Led::Blue => LED_BLUE.set_low(),
        Led::Green => LED_GREEN.set_low(),
    }
}

/// Get the state of a button
pub fn read_button(button: Button) -> gpio::Level {
    match button {
        Button::One => BUTTON_ONE.read(),
        Button::Two => BUTTON_TWO.read(),
    }
}

/// Call from a panic handler to flash the red LED quickly.
pub fn panic() -> ! {
    loop {
        led_on(Led::Red);
        ::delay(200);
        led_off(Led::Red);
        ::delay(200);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

fn enable_buttons() {
    BUTTON_ONE.set_direction(gpio::PinMode::InputPull(gpio::Level::High));
    BUTTON_TWO.set_direction(gpio::PinMode::InputPull(gpio::Level::High));
}

fn enable_leds() {
    LED_RED.set_direction(gpio::PinMode::Output);
    LED_BLUE.set_direction(gpio::PinMode::Output);
    LED_GREEN.set_direction(gpio::PinMode::Output);
}

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
