#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use atsamd51p;

#[entry]
fn main() -> ! {
    const PA_BIT_LED: u32 = 15;
    const PC_BIT_BUTTON1: u32 = 26;

    let peripherals = atsamd51p::Peripherals::take().unwrap();

    unsafe {
        peripherals.PORT.group0.dirset.write(|w| w.bits(1 << PA_BIT_LED));         // USER_LED = PA15 (Group0_15) = Output
        peripherals.PORT.group2.dirclr.write(|w| w.bits(1 << PC_BIT_BUTTON1));     // Button1 = PC26 (Group2_26) = Input
        peripherals.PORT.group2.pincfg[26].write(|w| w.bits(1 << 1));                // Button1 = PC26 (Group2_26) = Input Enable

        loop {
            let button1_input = peripherals.PORT.group2.in_.read().bits() & (1 << PC_BIT_BUTTON1);
            if button1_input > 0 {
                /* Button is not pressed */
                peripherals.PORT.group0.outset.write(|w| w.bits(1 << PA_BIT_LED));    // USER_LED = PA15 (Group0_15) = Set
            } else {
                /* Button is pressed */
                peripherals.PORT.group0.outclr.write(|w| w.bits(1 << PA_BIT_LED));    // USER_LED = PA15 (Group0_15) = Clr
            }
        }
    }
}
