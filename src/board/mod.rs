//! A board support library for the TI Stellaris Launchpad

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

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

use embedded_hal::digital::v2::OutputPin;
use tm4c123x_hal::gpio::{gpiof::*, GpioExt, Input, Output, PullUp, PushPull};
use tm4c123x_hal::sysctl::{
    Clocks, CrystalFrequency, Oscillator, PllOutputFrequency, SysctlExt, SystemClock,
};
use tm4c123x_hal::time::Hertz;

/// Represents the EK-LM4F120XL LaunchPad board, with the locations of the LEDs and buttons
/// predefined.
#[allow(non_snake_case)]
pub struct Board {
    /// The core peripherals on the LM4F120 / TM4C1233
    pub core_peripherals: tm4c123x_hal::CorePeripherals,
    /// Power gating for peripherals in the LM4F120 / TM4C1233
    pub power_control: tm4c123x_hal::sysctl::PowerControl,
    /// The pin used for the Red LED
    pub led_red: PF1<Output<PushPull>>,
    /// The pin used for the Blue LED
    pub led_blue: PF2<Output<PushPull>>,
    /// The pin used for the Green LED
    pub led_green: PF3<Output<PushPull>>,
    /// The pin used for Button One
    pub button_one: PF4<Input<PullUp>>,
    /// The pin used for Button Two
    pub button_two: PF0<Input<PullUp>>,
    /// GPIO control for GPIO port F
    pub portf_control: tm4c123x_hal::gpio::gpiof::GpioControl,

    // moved from the tm4c123x crate, with the exception of GPIO_PORTF -- those pins are moved to
    // the LEDs and buttons above.  sysctl is omitted, and only the power_control portion is
    // included (above) to allow the user to enable peripherals.
    #[doc = "WATCHDOG0"]
    pub WATCHDOG0: tm4c123x_hal::tm4c123x::WATCHDOG0,
    #[doc = "WATCHDOG1"]
    pub WATCHDOG1: tm4c123x_hal::tm4c123x::WATCHDOG1,
    #[doc = "GPIO_PORTA"]
    pub GPIO_PORTA: tm4c123x_hal::tm4c123x::GPIO_PORTA,
    #[doc = "GPIO_PORTB"]
    pub GPIO_PORTB: tm4c123x_hal::tm4c123x::GPIO_PORTB,
    #[doc = "GPIO_PORTC"]
    pub GPIO_PORTC: tm4c123x_hal::tm4c123x::GPIO_PORTC,
    #[doc = "GPIO_PORTD"]
    pub GPIO_PORTD: tm4c123x_hal::tm4c123x::GPIO_PORTD,
    #[doc = "SSI0"]
    pub SSI0: tm4c123x_hal::tm4c123x::SSI0,
    #[doc = "SSI1"]
    pub SSI1: tm4c123x_hal::tm4c123x::SSI1,
    #[doc = "SSI2"]
    pub SSI2: tm4c123x_hal::tm4c123x::SSI2,
    #[doc = "SSI3"]
    pub SSI3: tm4c123x_hal::tm4c123x::SSI3,
    #[doc = "UART0"]
    pub UART0: tm4c123x_hal::tm4c123x::UART0,
    #[doc = "UART1"]
    pub UART1: tm4c123x_hal::tm4c123x::UART1,
    #[doc = "UART2"]
    pub UART2: tm4c123x_hal::tm4c123x::UART2,
    #[doc = "UART3"]
    pub UART3: tm4c123x_hal::tm4c123x::UART3,
    #[doc = "UART4"]
    pub UART4: tm4c123x_hal::tm4c123x::UART4,
    #[doc = "UART5"]
    pub UART5: tm4c123x_hal::tm4c123x::UART5,
    #[doc = "UART6"]
    pub UART6: tm4c123x_hal::tm4c123x::UART6,
    #[doc = "UART7"]
    pub UART7: tm4c123x_hal::tm4c123x::UART7,
    #[doc = "I2C0"]
    pub I2C0: tm4c123x_hal::tm4c123x::I2C0,
    #[doc = "I2C1"]
    pub I2C1: tm4c123x_hal::tm4c123x::I2C1,
    #[doc = "I2C2"]
    pub I2C2: tm4c123x_hal::tm4c123x::I2C2,
    #[doc = "I2C3"]
    pub I2C3: tm4c123x_hal::tm4c123x::I2C3,
    #[doc = "GPIO_PORTE"]
    pub GPIO_PORTE: tm4c123x_hal::tm4c123x::GPIO_PORTE,
    #[doc = "PWM0"]
    pub PWM0: tm4c123x_hal::tm4c123x::PWM0,
    #[doc = "PWM1"]
    pub PWM1: tm4c123x_hal::tm4c123x::PWM1,
    #[doc = "QEI0"]
    pub QEI0: tm4c123x_hal::tm4c123x::QEI0,
    #[doc = "QEI1"]
    pub QEI1: tm4c123x_hal::tm4c123x::QEI1,
    #[doc = "TIMER0"]
    pub TIMER0: tm4c123x_hal::tm4c123x::TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: tm4c123x_hal::tm4c123x::TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: tm4c123x_hal::tm4c123x::TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: tm4c123x_hal::tm4c123x::TIMER3,
    #[doc = "TIMER4"]
    pub TIMER4: tm4c123x_hal::tm4c123x::TIMER4,
    #[doc = "TIMER5"]
    pub TIMER5: tm4c123x_hal::tm4c123x::TIMER5,
    #[doc = "WTIMER0"]
    pub WTIMER0: tm4c123x_hal::tm4c123x::WTIMER0,
    #[doc = "WTIMER1"]
    pub WTIMER1: tm4c123x_hal::tm4c123x::WTIMER1,
    #[doc = "ADC0"]
    pub ADC0: tm4c123x_hal::tm4c123x::ADC0,
    #[doc = "ADC1"]
    pub ADC1: tm4c123x_hal::tm4c123x::ADC1,
    #[doc = "COMP"]
    pub COMP: tm4c123x_hal::tm4c123x::COMP,
    #[doc = "CAN0"]
    pub CAN0: tm4c123x_hal::tm4c123x::CAN0,
    #[doc = "CAN1"]
    pub CAN1: tm4c123x_hal::tm4c123x::CAN1,
    #[doc = "WTIMER2"]
    pub WTIMER2: tm4c123x_hal::tm4c123x::WTIMER2,
    #[doc = "WTIMER3"]
    pub WTIMER3: tm4c123x_hal::tm4c123x::WTIMER3,
    #[doc = "WTIMER4"]
    pub WTIMER4: tm4c123x_hal::tm4c123x::WTIMER4,
    #[doc = "WTIMER5"]
    pub WTIMER5: tm4c123x_hal::tm4c123x::WTIMER5,
    #[doc = "USB0"]
    pub USB0: tm4c123x_hal::tm4c123x::USB0,
    #[doc = "GPIO_PORTA_AHB"]
    pub GPIO_PORTA_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTA_AHB,
    #[doc = "GPIO_PORTB_AHB"]
    pub GPIO_PORTB_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTB_AHB,
    #[doc = "GPIO_PORTC_AHB"]
    pub GPIO_PORTC_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTC_AHB,
    #[doc = "GPIO_PORTD_AHB"]
    pub GPIO_PORTD_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTD_AHB,
    #[doc = "GPIO_PORTE_AHB"]
    pub GPIO_PORTE_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTE_AHB,
    #[doc = "GPIO_PORTF_AHB"]
    pub GPIO_PORTF_AHB: tm4c123x_hal::tm4c123x::GPIO_PORTF_AHB,
    #[doc = "EEPROM"]
    pub EEPROM: tm4c123x_hal::tm4c123x::EEPROM,
    #[doc = "SYSEXC"]
    pub SYSEXC: tm4c123x_hal::tm4c123x::SYSEXC,
    #[doc = "HIB"]
    pub HIB: tm4c123x_hal::tm4c123x::HIB,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: tm4c123x_hal::tm4c123x::FLASH_CTRL,
    #[doc = "UDMA"]
    pub UDMA: tm4c123x_hal::tm4c123x::UDMA,
}

// a moderately sane default, but will be replaced by Board::new()
static mut CLOCKS: Clocks = Clocks {
    osc: Hertz(16_000_000),
    sysclk: Hertz(66_666_667),
};

/// Get the current clock rate of the CPU
pub fn clocks() -> &'static Clocks {
    unsafe { &CLOCKS }
}

impl Board {
    /// Initialise everything on the board - FPU, PLL, SysTick, GPIO and the LEDs
    /// and buttons. Should be pretty much the first call you make in `main()`.
    /// Doesn't init the UART - that's separate.
    pub(crate) fn new() -> Board {
        let core_peripherals = tm4c123x_hal::CorePeripherals::take().unwrap();
        let peripherals = tm4c123x_hal::Peripherals::take().unwrap();
        let mut sysctl = peripherals.SYSCTL.constrain();

        // this might belong in tm4c123x_hal, but allow FPU usage
        unsafe {
            core_peripherals.SCB.cpacr.modify(|d| {
                d | (0x3 /* full */ << 20/* CP10 privilege */)
                    | (0x3 /* full */ << 22/* CP11 privilege */)
            });
        }

        sysctl.clock_setup.oscillator = Oscillator::Main(
            CrystalFrequency::_16mhz,
            SystemClock::UsePll(PllOutputFrequency::_66_67mhz),
        );
        unsafe {
            CLOCKS = sysctl.clock_setup.freeze();
        }
        let mut pins = peripherals.GPIO_PORTF.split(&sysctl.power_control);
        let led_red = pins.pf1.into_push_pull_output();
        let led_blue = pins.pf2.into_push_pull_output();
        let led_green = pins.pf3.into_push_pull_output();
        let button_one = pins.pf4.into_pull_up_input();
        let button_two = pins.pf0.unlock(&mut pins.control).into_pull_up_input();

        Board {
            core_peripherals,
            power_control: sysctl.power_control,
            led_red,
            led_blue,
            led_green,
            button_one,
            button_two,
            portf_control: pins.control,
            WATCHDOG0: peripherals.WATCHDOG0,
            WATCHDOG1: peripherals.WATCHDOG1,
            GPIO_PORTA: peripherals.GPIO_PORTA,
            GPIO_PORTB: peripherals.GPIO_PORTB,
            GPIO_PORTC: peripherals.GPIO_PORTC,
            GPIO_PORTD: peripherals.GPIO_PORTD,
            SSI0: peripherals.SSI0,
            SSI1: peripherals.SSI1,
            SSI2: peripherals.SSI2,
            SSI3: peripherals.SSI3,
            UART0: peripherals.UART0,
            UART1: peripherals.UART1,
            UART2: peripherals.UART2,
            UART3: peripherals.UART3,
            UART4: peripherals.UART4,
            UART5: peripherals.UART5,
            UART6: peripherals.UART6,
            UART7: peripherals.UART7,
            I2C0: peripherals.I2C0,
            I2C1: peripherals.I2C1,
            I2C2: peripherals.I2C2,
            I2C3: peripherals.I2C3,
            GPIO_PORTE: peripherals.GPIO_PORTE,
            PWM0: peripherals.PWM0,
            PWM1: peripherals.PWM1,
            QEI0: peripherals.QEI0,
            QEI1: peripherals.QEI1,
            TIMER0: peripherals.TIMER0,
            TIMER1: peripherals.TIMER1,
            TIMER2: peripherals.TIMER2,
            TIMER3: peripherals.TIMER3,
            TIMER4: peripherals.TIMER4,
            TIMER5: peripherals.TIMER5,
            WTIMER0: peripherals.WTIMER0,
            WTIMER1: peripherals.WTIMER1,
            ADC0: peripherals.ADC0,
            ADC1: peripherals.ADC1,
            COMP: peripherals.COMP,
            CAN0: peripherals.CAN0,
            CAN1: peripherals.CAN1,
            WTIMER2: peripherals.WTIMER2,
            WTIMER3: peripherals.WTIMER3,
            WTIMER4: peripherals.WTIMER4,
            WTIMER5: peripherals.WTIMER5,
            USB0: peripherals.USB0,
            GPIO_PORTA_AHB: peripherals.GPIO_PORTA_AHB,
            GPIO_PORTB_AHB: peripherals.GPIO_PORTB_AHB,
            GPIO_PORTC_AHB: peripherals.GPIO_PORTC_AHB,
            GPIO_PORTD_AHB: peripherals.GPIO_PORTD_AHB,
            GPIO_PORTE_AHB: peripherals.GPIO_PORTE_AHB,
            GPIO_PORTF_AHB: peripherals.GPIO_PORTF_AHB,
            EEPROM: peripherals.EEPROM,
            SYSEXC: peripherals.SYSEXC,
            HIB: peripherals.HIB,
            FLASH_CTRL: peripherals.FLASH_CTRL,
            UDMA: peripherals.UDMA,
        }
    }
}

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

/// Call from a panic handler to flash the red LED quickly.
pub fn panic() -> ! {
    use embedded_hal::blocking::delay::DelayMs;
    let core_peripherals = unsafe { tm4c123x_hal::CorePeripherals::steal() };
    let p = unsafe { tm4c123x_hal::Peripherals::steal() };
    let pins = p.GPIO_PORTF.split(&p.SYSCTL.constrain().power_control);

    let mut delay = tm4c123x_hal::delay::Delay::new(core_peripherals.SYST, unsafe { &CLOCKS });
    let mut led_red = pins.pf1.into_push_pull_output();
    loop {
        let _ = led_red.set_high();
        delay.delay_ms(200u32);
        let _ = led_red.set_low();
        delay.delay_ms(200u32);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
