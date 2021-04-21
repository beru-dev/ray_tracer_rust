use std::ops::{Add, Sub, Mul};
use crate::approx_equal::*;

#[derive(Debug, Copy, Clone)]
pub struct Color(pub f64, pub f64, pub f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        return Color(red, green, blue);
    }

    fn convert_color_value(normalized_color: f64) -> usize {
        if normalized_color > 1.0 {
            return 255;
        } else if normalized_color < 0.0 {
            return 0;
        }
        (normalized_color * 255.0).round() as usize
    }

    pub fn integer_color_data(&self) -> [usize; 3] {
        [
            Color::convert_color_value(self.0),
            Color::convert_color_value(self.1),
            Color::convert_color_value(self.2)
        ]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        use approx_equal::equal;
        equal(self.0, other.0) &&
        equal(self.1, other.1) &&
        equal(self.2, other.2)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) ->  Self {
        Color(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Self) -> Self {
        Color(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2
        )
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color(
            self.0 * other,
            self.1 * other,
            self.2 * other
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c.0, -0.5);
        assert_eq!(c.1, 0.4);
        assert_eq!(c.2, 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(1.6, 0.7, 1.0);

        assert_eq!(c1 + c2, expected);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let expected = Color::new(0.2, 0.5, 0.5);

        assert_eq!(c1 - c2, expected);
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let expected = Color::new(0.4, 0.6, 0.8);

        assert_eq!(c * 2.0, expected);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let expected = Color::new(0.9, 0.2, 0.04);

        assert_eq!(c1 * c2, expected);
    }

    #[test]
    fn scaling_color_val() {
        assert_eq!(Color::convert_color_value(1.1), 255);
        assert_eq!(Color::convert_color_value(1.0), 255);
        assert_eq!(Color::convert_color_value(0.0), 0);
        assert_eq!(Color::convert_color_value(-1.0), 0);
        assert_eq!(Color::convert_color_value(0.99999), 255);
        assert_eq!(Color::convert_color_value(0.5), 128);
    }
}