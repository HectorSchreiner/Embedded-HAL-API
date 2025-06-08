use arduino_hal::{hal::port::{PC1, PC2}, port::{mode::Analog, Pin}, Adc};

use crate::{adapters::arduino_uno::joystick, domains::{joystick::JoystickReader, math::{clamp, Vec2}, types::Deadzone2Axis}};

pub struct Joystick {
    pin_x: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC1>,
    pin_y: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC2>,
}

impl JoystickReader<f32> for Joystick {
    fn read_raw(&mut self) -> Vec2<f32> {
        Vec2::new(1.0, 1.0)
    }

    /// Read the Joystic values, and returns a `Vec2<u16>`
    ///
    /// **Example**:
    /// ```no_run
    ///     let (x, y) = joystick.read_analog(&mut adc);
    ///     console_writeln!("X: {}, Y: {}", x, y);
    /// ```
    fn read_analog(&mut self, adc: &mut Adc) -> Vec2<u16> {
        let x = self.pin_x.analog_read(adc);
        let y = self.pin_y.analog_read(adc);
        Vec2::new(x, y)
    }

    /// returns the read_analog, but normalised to a value from 0-1000
    fn read_analog_normalized(&mut self, adc: &mut Adc, deadzone: Deadzone2Axis) -> Vec2<u16> {
        let mut x_lower = deadzone.x_lower;
        let mut x_middle = deadzone.x_middle;
        let mut x_upper = deadzone.x_upper;

        let mut y_lower = deadzone.y_lower;
        let mut y_middle = deadzone.y_middle;
        let mut y_upper = deadzone.y_upper;

        let Vec2 {x, y} = Self::read_analog(self, adc);
        
                
        todo!()
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
}