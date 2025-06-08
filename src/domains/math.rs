use libm::sqrt;
use num_traits::{Num, NumCast};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2<f64> {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Vec2<u16> {
    pub fn empty() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct NormalizeError;

/// Normalized2D is a form of Vec2. The Normalized2D struct holds a Vec2 of type f64, which length is normalized to a size of 1.0.
pub struct Normalized2D {
    vec: Vec2<f64>
}

impl Normalized2D {
    pub fn new(vec: Vec2<f64>) -> Result<Self, NormalizeError> {
        let length_squared = vec.x * vec.x + vec.y * vec.y;

        if length_squared == 0.0 {
            return Err(NormalizeError);
        }

        let length = sqrt(length_squared);
        let normalized = Vec2 {
            x: vec.x / length,
            y: vec.y / length,
        };

        Ok(Self { vec: normalized })
    }

    pub fn get_vec(&self) -> Vec2<f64> {
        self.vec
    }
}

impl TryFrom<(f64, f64)> for Normalized2D {
    type Error = NormalizeError;
    
    fn try_from(value: (f64, f64)) -> Result<Self, Self::Error> {
        Self::new(Vec2::new(value.0, value.1))
    }    
}

pub struct MapValueError;

/// Helper function to map a value.
pub fn map_value<T>(value: T, in_min: T, in_max: T, out_min: T, out_max: T) -> Result<T, MapValueError>
where
    T: Num + NumCast + Copy + PartialOrd + PartialEq,
{
    // needed to make code generic
    let value: f64 = NumCast::from(value).unwrap();
    let in_min: f64= NumCast::from(in_min).unwrap();
    let in_max: f64 = NumCast::from(in_max).unwrap();
    let out_min: f64 = NumCast::from(out_min).unwrap();
    let out_max: f64 = NumCast::from(out_max).unwrap();

    let result = if (in_max - in_min).abs() < f64::EPSILON {
        out_min
    } else {
        (value - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
    };

    T::from(result).ok_or(MapValueError)
}

/// Clamps an *input value* between a *min* and *max*.
///
/// **Example:**
/// ```no_run
///     // clamps the variable *value* between 0.0 and 1.0
///     Self::clamp(value, 0.0, 1.1)
/// ```
pub fn clamp<T>(val: T, min: T, max: T) -> T 
where
    T: Num + NumCast + Copy + PartialOrd + PartialEq,
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
