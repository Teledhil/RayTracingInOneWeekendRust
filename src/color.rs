use std::fmt;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub const WHITE: Color = Color::const_new(1.0, 1.0, 1.0);
pub const BLACK: Color = Color::const_new(0.0, 0.0, 0.0);

const MAX_CLAMP: f64 = 0.999;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let r: u8 = (256_f64 * self.x().clamp(0_f64, MAX_CLAMP)) as u8;
        let g: u8 = (256_f64 * self.y().clamp(0_f64, MAX_CLAMP)) as u8;
        let b: u8 = (256_f64 * self.z().clamp(0_f64, MAX_CLAMP)) as u8;

        writeln!(f, "{r} {g} {b}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        let value = (256_f64 * MAX_CLAMP) as u16;
        assert_eq!(255, value);
    }
}
