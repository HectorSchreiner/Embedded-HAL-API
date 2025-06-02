#![no_std]
#![no_main]

#[macro_use]
mod print;

use arduino_hal::adc::AdcSettings;
use print::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, AdcSettings::default());

    let joystick_pin_x = pins.a1.into_analog_input(&mut adc);
    let joystick_pin_y = pins.a2.into_analog_input(&mut adc);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    // Set up global console for use in macros
    avr_device::interrupt::free(|cs| {
        CONSOLE.borrow(cs).replace(Some(serial));
    });

    loop {
        let x = joystick_pin_x.analog_read(&mut adc);
        let y = joystick_pin_y.analog_read(&mut adc);

        println!("X: {}, Y: {}", x, y);
    }
}
