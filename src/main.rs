#![no_std]
#![no_main]

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
extern crate cortex_m;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");
    // GPIO pins start at address 0x5000_0000
    const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32; // PIN_CNF[21] register
    const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32; // PIN_CNF[28] register
    const DIR_OUTPUT_POS: u32 = 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
    unsafe {
        // use write_volatile to make the compiler does not optimize this out
        // configure both pins as output
        write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED);
        write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED);
    }

    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32; // OUT register
    const GPIO0_OUT_ROW1_POS: u32 = 21;
    let mut is_on: bool = false;
    loop {
        unsafe {
            // drive the pin high or low depending on the `is_on` value
            write_volatile(GPIO0_OUT_ADDR, (is_on as u32) << GPIO0_OUT_ROW1_POS);
            // rudimentary sleep
            for _ in 0..400_000 {
                nop();
            }
            is_on = !is_on;
        }

        if is_on {
            rprintln!("BLINK");
        }
    }
}
