use arduino_hal::{hal::port::{PC1, PC2}, port::{mode::Analog, Pin}, Adc};

use crate::{adapters::arduino_uno::joystick, domains::{joystick::JoystickReader, math::Vec2}};

pub struct Joystick {
    pin_x: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC1>,
    pin_y: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC2>,
}

impl JoystickReader<f32> for Joystick {
    fn read_raw(&mut self) -> Vec2<f32> {
        Vec2::new(1.0, 1.0)
    }
}

impl Joystick {
    /// Initialize the Joystic.
    ///
    /// **Example**:
    /// ```no_run
    ///    let dp = arduino_hal::Peripherals::take().unwrap();
    ///    let pins = arduino_hal::pins!(dp);
    ///    let mut adc = arduino_hal::Adc::new(dp.ADC, AdcSettings::default());
    ///
    ///    let mut joystick = Joystick::new(
    ///        pins.a1.into_analog_input(&mut adc),
    ///        pins.a2.into_analog_input(&mut adc)
    ///    );
    /// ```

    pub fn new(
        pin_x: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC1>,
        pin_y: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC2>
    ) -> Self {
        Self { pin_x, pin_y }
    }
        /// Read the Joystic values, and returns a `(u16, u16)`.
    ///
    /// **Example**:
    /// ```no_run
    ///     let (x, y) = joystick.read_analog(&mut adc);
    ///     console_writeln!("X: {}, Y: {}", x, y);
    /// ```

    pub fn read_analog(&mut self, adc: &mut Adc) -> (u16, u16) {
        let x = self.pin_x.analog_read(adc);
        let y = self.pin_y.analog_read(adc);
        (x, y)
    }

    /// Read the Joystic values, and returns a tuple with values within the spefied range.
    pub fn read_normalized(&mut self, adc: &mut Adc, x_deadzone_upper: f32, x_deadzone_lower: f32, y_deadzone_upper: f32, ) -> (f32, f32) {
        let analog = Self::read_analog(self, adc);

        (0.0, 0.0)
    }
}