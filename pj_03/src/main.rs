#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use atsamd_hal::{prelude::*, gpio::v2::Pins};

#[entry]
fn main() -> ! {
    let peripherals = atsamd_hal::pac::Peripherals::take().unwrap();
    // let peripherals = atsamd_hal::target_device::Peripherals::take().unwrap();   // same

    // let mut pins = peripherals.PORT.split();      // gpio v1 is deprecated
    let pins = Pins::new(peripherals.PORT);     // so, use gpio v2
    let mut led = pins.pa15.into_push_pull_output();       // USER_LED = PA15 = Output
    let button1 = pins.pc26.into_floating_input(); // Button1 = PC26 = Input

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
