use crate::domains::math::*;

pub struct Deadzone2Axis {
    pub x_lower: u16,
    pub x_middle:u16,
    pub x_upper: u16,
    pub y_lower: u16,
    pub y_middle:u16,
    pub y_upper: u16
}

impl Deadzone2Axis {
    pub fn new(    
    x_lower: u16,
    x_middle:u16,
    x_upper: u16,
    y_lower: u16,
    y_middle:u16,
    y_upper: u16) -> Self {
        Self {
            x_lower,
            x_middle,
            x_upper,
            y_lower,
            y_middle,
            y_upper
        }
    }
/*
    pub fn normalize(&self, input: (f64, f64)) -> Result<Normalized2D, NormalizeError> {
        let out_min = 0.0;
        let out_max = 1.0;

        let mapped_x = clamp(
            map_float(input.0, self.x_lower, self.x_upper, out_min, out_max),
            out_min,
            out_max,
        );
        let mapped_y = clamp(
            map_float(input.1, self.y_lower, self.y_upper, out_min, out_max),
            out_min,
            out_max,
        );

        Normalized2D::try_from((mapped_x, mapped_y))
    }
    */
}
