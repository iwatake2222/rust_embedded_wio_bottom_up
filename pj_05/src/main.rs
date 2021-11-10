#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::prelude::*;


#[entry]
fn main() -> ! {
    let mut peripherals = wio::pac::Peripherals::take().unwrap();
    let core = wio::pac::CorePeripherals::take().unwrap();

    /* Initialize clock controller */
    let mut clocks = wio::hal::clock::GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    /* Get delay driver based on SysTick */
    let mut delay = wio::hal::delay::Delay::new(core.SYST, &mut clocks);

    /* Set pin configuration */
    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    
    loop {
        /* Blink LED */
        delay.delay_ms(1000u16);
        led.set_high().unwrap();
        delay.delay_ms(1000u16);
        led.set_low().unwrap();
    }
}
