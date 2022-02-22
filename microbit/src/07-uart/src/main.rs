#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // writeln!(serial, "The quick brown fox jumps over the lazy dog.");
    // nb::block!(serial.flush()).unwrap();

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        let byte = nb::block!(serial.read()).unwrap();
        if byte == b'\r' {
            buffer.reverse();
            writeln!(serial, "{}", core::str::from_utf8(&buffer).unwrap()).unwrap();
            nb::block!(serial.flush()).unwrap();
            buffer.clear();
        } else {
            match buffer.push(byte) {
                Ok(_) => {}
                Err(_) => {
                    buffer.reverse();
                    writeln!(
                        serial,
                        "Exceded buffer size, here is what you had so far:{}",
                        core::str::from_utf8(&buffer).unwrap()
                    )
                    .unwrap();
                    nb::block!(serial.flush()).unwrap();
                    buffer.clear();
                }
            }
        }
    }
}
