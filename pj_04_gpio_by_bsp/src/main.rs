#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = wio::pac::Peripherals::take().unwrap();

    let mut pins = wio::Pins::new(peripherals.PORT);
    let mut led = pins.user_led.into_push_pull_output(&mut pins.port);
    let button1 = pins.button1.into_floating_input(&mut pins.port);
    
    loop {
        if button1.is_high().unwrap() {
            /* Button is not pressed */
            led.set_high().unwrap();
        } else {
            /* Button is pressed */
            led.set_low().unwrap();
        }
    }
}