use arduino_hal::{hal::port::{PC1, PC2}, Adc};

pub struct Joystick {
    pin_x: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC1>,
    pin_y: arduino_hal::port::Pin<arduino_hal::port::mode::Analog, PC2>,
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

pub struct Normalized2D {
    pub x: f32,
    pub y: f32,
}

impl Normalized2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Deadzone2Axis {
    x_lower: f32,
    x_upper: f32,
    y_lower: f32,
    y_upper: f32,
}

impl Deadzone2Axis {
    pub fn new(x_lower: usize, x_upper: usize, y_lower: usize, y_upper: usize) -> Self {
        Self {
            x_lower: x_lower as f32,
            x_upper: x_upper as f32,
            y_lower: y_lower as f32,
            y_upper: y_upper as f32,
        }
    }

    pub fn normalize(&self, input: (f32, f32)) -> Normalized2D {
        let out_min = 0.0;
        let out_max = 1.0;

        let mapped_x = Self::clamp(
            Self::map_float(input.0, self.x_lower, self.x_upper, out_min, out_max),
            out_min,
            out_max,
        );
        let mapped_y = Self::clamp(
            Self::map_float(input.1, self.y_lower, self.y_upper, out_min, out_max),
            out_min,
            out_max,
        );

        Normalized2D::new(mapped_x, mapped_y)
    }

    fn map_float(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        if (in_max - in_min).abs() < f32::EPSILON {
            out_min
        } else {
            (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
        }
    }

    fn clamp(val: f32, min: f32, max: f32) -> f32 {
        if val < min {
            min
        } else if val > max {
            max
        } else {
            val
        }
    }
}
