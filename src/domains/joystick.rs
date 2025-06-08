use arduino_hal::Adc;

use crate::domains::math::{self, Vec2}; 
use crate::domains::types::Deadzone2Axis;

pub trait JoystickReader<T> {
    fn read_raw(&mut self) -> Vec2<T>;
    fn read_analog(&mut self, adc: &mut Adc) -> Vec2<u16>;
    fn read_analog_normalized(&mut self, adc: &mut Adc, deadzone: Deadzone2Axis) -> Vec2<u16>;
}
