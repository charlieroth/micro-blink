#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting...");
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;
    loop {
        let _ = row1.set_low();
        timer.delay_ms(400);
        let _ = row1.set_high();
        rprintln!("BLINK");
        timer.delay_ms(400);
    }
}
