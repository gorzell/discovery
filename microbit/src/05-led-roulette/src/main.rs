#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::display::blocking::Display;
use microbit::hal::prelude::*;
use microbit::hal::timer::Timer;
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    let mut display = Display::new(board.display_pins);

    let mut pattern = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    const DELAY: u32 = 50;

    loop {
        for i in 0..pattern[0].len() {
            pattern[0][i] = 1;
            display.show(&mut timer, pattern, DELAY);
            pattern[0][i] = 0;
        }

        for i in 1..pattern.len() - 1 {
            pattern[i][pattern[i].len() - 1] = 1;
            display.show(&mut timer, pattern, DELAY);
            pattern[i][pattern[i].len() - 1] = 0;
        }

        for i in (0..pattern[0].len()).rev() {
            pattern[4][i] = 1;
            display.show(&mut timer, pattern, DELAY);
            pattern[4][i] = 0;
        }
        for i in (1..pattern.len() - 1).rev() {
            pattern[i][0] = 1;
            display.show(&mut timer, pattern, DELAY);
            pattern[i][0] = 0;
        }
    }
}
