use crate::domains::math::{self, Vec2};

pub trait JoystickReader<T> {
    fn read_raw(&mut self) -> Vec2<T>;
}
