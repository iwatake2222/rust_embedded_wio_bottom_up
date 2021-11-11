#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {

    unsafe {
        /* Register address */
        const PORT_ADDRESS: u32 = 0x4100_8000;
        const PA_ADDRESS: u32 = PORT_ADDRESS + 0x80 * 0;
        const PC_ADDRESS: u32 = PORT_ADDRESS + 0x80 * 2;
        const PA_DIRSET: u32 = PA_ADDRESS + 0x08;
        const PA_OUTCLR: u32 = PA_ADDRESS + 0x14;
        const PA_OUTSET: u32 = PA_ADDRESS + 0x18;
        const PC_DIRCLR: u32 = PC_ADDRESS + 0x04;
        const PC_PINCFG26: u32 = PC_ADDRESS + 0x40 + 26;
        const PC_IN: u32 = PC_ADDRESS + 0x20;
        const PA_BIT_LED: u32 = 15;
        const PC_BIT_BUTTON1: u32 = 26;

        /* Set pin configuration */
        *(PA_DIRSET as *mut u32) = 1 << PA_BIT_LED;         // USER_LED = PA15 = Output
        *(PC_DIRCLR as *mut u32) = 1 << PC_BIT_BUTTON1;     // Button1 = PC26 = Input
        *(PC_PINCFG26 as *mut u32) |= 1 << 1;               // Button1 = PC26 = Input Enable

        loop {
            let button1_input = *(PC_IN as *mut u32) & 1 << PC_BIT_BUTTON1;
            if button1_input > 0 {
                /* Button is not pressed */
                *(PA_OUTSET as *mut u32) = 1 << PA_BIT_LED;
            } else {
                /* Button is pressed */
                *(PA_OUTCLR as *mut u32) = 1 << PA_BIT_LED;
            }
        }
    }
}
