#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::{entry};

#[entry]
fn main() -> ! {

    unsafe {
        const PA_DIRSET: u32 = 0x41008008;
        const PA_OUTSET: u32 = 0x41008018;
        *(PA_DIRSET as *mut u32) = 1 << 15;
        *(PA_OUTSET as *mut u32) = 1 << 15;
    }

    loop {
    }
}
