use crate::domains::math;

pub trait JoystickReader {
    pub fn read_raw(&mut self) -> Vec2;
}
