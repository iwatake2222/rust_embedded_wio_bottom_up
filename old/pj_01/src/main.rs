#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m;

#[entry]
fn main() -> ! {

    unsafe {
        /* Register address */
        const PORT_ADDRESS: u32 = 0x4100_8000;
        const PA_ADDRESS: u32 = PORT_ADDRESS + 0x80 * 0;
        const PA_DIRSET: u32 = PA_ADDRESS + 0x08;
        const PA_OUTCLR: u32 = PA_ADDRESS + 0x14;
        const PA_OUTSET: u32 = PA_ADDRESS + 0x18;
        const PA_BIT_LED: u32 = 15;

        /* Set pin configuration */
        *(PA_DIRSET as *mut u32) = 1 << PA_BIT_LED;         // USER_LED = PA15 = Output

        /* Get delay driver based on SysTick */
        let core = cortex_m::Peripherals::take().unwrap();
        const AHB_CLK_MHZ: u32 = (120.0 / 2.5) as u32;
        let mut delay = cortex_m::delay::Delay::new(core.SYST, AHB_CLK_MHZ * 1000000);
        
        loop {
            /* Blink LED */
            delay.delay_ms(1000);
            *(PA_OUTSET as *mut u32) = 1 << PA_BIT_LED;
            delay.delay_ms(1000);
            *(PA_OUTCLR as *mut u32) = 1 << PA_BIT_LED;
        }
    }
}
