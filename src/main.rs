#![no_std]
#![no_main]

use core::pin::Pin;
use arduino_hal::adc::*;
use arduino_hal::hal::port::{PC1, PC2};
use panic_halt as _;

mod console;
mod domains;
mod adapters;
mod mock;
mod ports;

use crate::adapters::arduino_uno::joystick::*;
use crate::console::CONSOLE;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, AdcSettings::default());

    let mut joystick = Joystick::new(
        pins.a1.into_analog_input(&mut adc),
        pins.a2.into_analog_input(&mut adc)
    );

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    console::console_init(serial);

    loop {
        let (x, y) = joystick.read_analog(&mut adc);
        console_writeln!("X: {}, Y: {}", x, y);
    }
}
