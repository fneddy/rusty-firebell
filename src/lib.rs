#![no_std]
#![feature(lang_items)]

pub use avr_device::entry;
pub use avr_device::atmega328pb as device;

extern crate embedded_hal as hal;
extern crate nb;

pub mod prelude {
    pub use hal::digital::v2::*;
}

pub mod pins;
