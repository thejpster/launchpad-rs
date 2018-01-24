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

/// A physical button at the bottom of the Stellaris Launchpad
pub struct Button {
    pin: gpio::PinPort,
}

/// An LED on the Stellaris Launchpad. There are three in a single RGB unit.
pub struct Led {
    pin: gpio::PinPort,
}

/// The set of all LEDs on the Stellaris Launchpad. There are three in an RGB unit.
pub struct Leds {
    /// The red LED
    pub red: Option<Led>,
    /// The blue LED
    pub blue: Option<Led>,
    /// The green LED
    pub green: Option<Led>,
}

/// The set of all buttons on the Stellaris Launchpad. There are two at the bottom of the board.
pub struct Buttons {
    /// The left hand button
    pub one: Option<Button>,
    /// The right hand button
    pub two: Option<Button>,
}

/// The set of all spare GPIO pins on the Stellaris Launchpad (that is, not
/// used by LEDs, buttons or other hard-wired peripherals).
pub struct Pins {
    /// Pin PA0 on JXX.YY
    pub pa0: Option<gpio::PinPort>,
    /// Pin PA1 on JXX.YY
    pub pa1: Option<gpio::PinPort>,
}

/// The set of all things on a Stellaris Launchpad, including buttons, LEDs
/// and spare I/O pins.
pub struct Board {
    /// All the LEDs
    pub leds: Leds,
    /// All the buttons
    pub buttons: Buttons,
    /// All the spare pins (that aren't LEDs or buttons)
    pub pins: Pins,
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

// /// The pin used for the Red LED
// pub const LED_RED: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin1);
// /// The pin used for the Blue LED
// pub const LED_BLUE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin2);
// /// The pin used for the Green LED
// pub const LED_GREEN: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin3);
// /// The pin used for Button One
// pub const BUTTON_ONE: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin0);
// /// The pin used for Button Two
// pub const BUTTON_TWO: gpio::PinPort = gpio::PinPort::PortF(gpio::Pin::Pin4);

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Initialise everything on the board - FPU, PLL, SysTick, GPIO and the LEDs
/// and buttons. Should be pretty much the first call you make in `main()`.
/// Doesn't init the UART - that's separate.
pub fn init() -> Board {
    let mut pins = gpio::take().unwrap();
    fpu::init();
    pll::init(pll::ClockSpeed::Speed66MHz);
    systick::init();
    let mut board = Board {
        leds: Leds {
            red: pins.pf1.take().map(|pin| Led { pin }),
            blue: pins.pf2.take().map(|pin| Led { pin }),
            green: pins.pf3.take().map(|pin| Led { pin }),
        },
        buttons: Buttons {
            one: pins.pf0.take().map(|pin| Button { pin }),
            two: pins.pf4.take().map(|pin| Button { pin }),
        },
        pins: Pins {
            pa0: pins.pa0.take(),
            pa1: pins.pa1.take(),
        },
    };
    if let &mut Some(ref mut button) = &mut board.buttons.one {
        button
            .pin
            .set_direction(gpio::PinMode::InputPull(gpio::Level::High));
    }
    if let &mut Some(ref mut button) = &mut board.buttons.two {
        button
            .pin
            .set_direction(gpio::PinMode::InputPull(gpio::Level::High));
    }
    if let &mut Some(ref mut led) = &mut board.leds.red {
        led.pin.set_direction(gpio::PinMode::Output);
    }
    if let &mut Some(ref mut led) = &mut board.leds.green {
        led.pin.set_direction(gpio::PinMode::Output);
    }
    if let &mut Some(ref mut led) = &mut board.leds.blue {
        led.pin.set_direction(gpio::PinMode::Output);
    }
    board
}

impl Led {
    /// Turn an LED on
    pub fn on(&mut self) {
        self.pin.set_high()
    }

    /// Turn an LED off
    pub fn off(&mut self) {
        self.pin.set_low()
    }

    /// Switch to timer/capture mode.
    pub fn enable_pwm(&mut self) {
        self.pin.set_direction(gpio::PinMode::Peripheral);
        self.pin.enable_ccp();
    }
}

impl Button {
    /// Read the button state
    pub fn is_pushed(&self) -> bool {
        self.pin.read() == gpio::Level::Low
    }
}

/// Call from a panic handler to flash the red LED quickly.
pub fn panic() -> ! {
    loop {
        // How do we steal the LED pins here?
        // Probably have to use unsafe.
        // led_on(Led::Red);
        ::delay(200);
        // led_off(Led::Red);
        ::delay(200);
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
