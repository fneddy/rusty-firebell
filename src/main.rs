#![no_std]
#![no_main]

use panic_halt as _;

use rusty_firebell::prelude::*;
use rusty_firebell::pins;

#[rusty_firebell::entry]
fn main() -> ! {
    let mut pb0 = pins::PB0::as_output();
    pb0.set_high();
    loop {}
}
