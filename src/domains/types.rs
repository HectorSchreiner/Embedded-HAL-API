use crate::domains::math::*;

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

        //Normalized2D::new(mapped_x, mapped_y)
    }
}
