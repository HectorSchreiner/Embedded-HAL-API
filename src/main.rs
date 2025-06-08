#![no_std]
#![no_main]

use arduino_hal::adc::*;
use panic_halt as _;

mod console;
mod domains;
mod adapters;
mod mock;
mod ports;

use crate::adapters::arduino_uno::joystick::*;
use crate::console::CONSOLE;
use crate::domains::joystick::JoystickReader;

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
        let pos = joystick.read_analog(&mut adc);
        console_writeln!("X: {}, Y: {}", pos.x, pos.y);
    }
}
