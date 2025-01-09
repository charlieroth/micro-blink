#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::gpio::Level;
use nrf52833_hal::{
    self as hal,
    gpio::{
        p0::{P0_21, P0_28},
        Output, PushPull,
    },
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
extern crate cortex_m;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");

    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0); // Get `P0`, 0x5000_0000
    let _col1: P0_28<Output<PushPull>> = port0.p0_28.into_push_pull_output(Level::Low); // Ground `PIN_CNF[28]`
    let mut row1: P0_21<Output<PushPull>> = port0.p0_21.into_push_pull_output(Level::Low); // Ground `PIN_CNF[28]`, make mutable

    // blinking loop
    let mut is_on: bool = false;
    loop {
        let _ = row1.set_state(PinState::from(is_on));
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
        if is_on {
            rprintln!("BLINK");
        }
    }
}
