use num_traits::Float;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    x: T,
    y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1
        }
    }
}

pub struct NormalizeError(pub String);

/// Normalized2D is a form of Vec2. The Normalized2D struct holds a Vec2, which length is normalized to a size of 1.0.
pub struct Normalized2D<T: num_traits::Float> {
    vec: Vec2<T>
}

impl<T: num_traits::Float> Normalized2D<T> {
    pub fn new(vec: Vec2<T>) -> Self {
        Self::normalize(&mut self);
        Self { vec }
    }

    fn normalize(&mut self) -> Result<(), NormalizeError>{
        let length = Self::len(&self);
        if length > T::zero() {
            self.vec.x = self.vec.x / length;
            self.vec.y = self.vec.y / length;
        }
    }

    fn len(&self) -> T {
        (self.vec.x * self.vec.x + self.vec.y * self.vec.y).sqrt()
    }

    pub fn get_vec(&self) -> Vec2<T> {
        self.vec
    }
}

/// Helper function to map a value
pub fn map_float(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    if (in_max - in_min).abs() < f32::EPSILON {
        out_min
    } else {
        (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
    }
}

/// Clamps an *input value* between a *min* and *max*.
///
/// **Example:**
/// ```no_run
///     // clamps the variable *value* between 0.0 and 1.0
///     Self::clamp(value, 0.0, 1.1)
/// ```
pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
