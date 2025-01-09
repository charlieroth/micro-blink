#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
extern crate cortex_m;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");

    let p = Peripherals::take().unwrap();
    // write to `PIN_CNF[21]`, pin direction as output
    p.P0.pin_cnf[21].write(|w| w.dir().output());
    // write to `PIN_CNF[28]`, pin direction as output
    p.P0.pin_cnf[28].write(|w| w.dir().output());

    let mut is_on: bool = false;
    loop {
        // write to `OUT`, write bit `is_on` to Bit/Pin 21
        p.P0.out.write(|w| w.pin21().bit(is_on));
        for _ in 0..200_000 {
            nop();
        }
        is_on = !is_on;
        if is_on {
            rprintln!("BLINK");
        }
    }
}
