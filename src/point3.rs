use std::ops::{Add, Sub};
use crate::approx_equal::*;
use crate::vector3::*;

#[derive(Debug, Copy, Clone)]
pub struct Point3(pub f64, pub f64, pub f64, pub f64);

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3(x, y, z, 1.0)
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vector3) -> Point3 {
        Point3::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub for Point3 {
    type Output = Vector3;

    fn sub(self, other: Point3) -> Vector3 {
        Vector3::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vector3) -> Point3 {
        Point3::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        use approx_equal::equal;
        equal(self.0, other.0) &&
        equal(self.1, other.1) &&
        equal(self.2, other.2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_plus_vector_is_point() {
        let p = Point3::new(3.0, -2.0, 5.0) + Vector3::new(-2.0, 3.0, 1.0);
        assert_eq!(p, Point3::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn point_minus_point_is_vector() {
        let p1 = Point3::new(3.0, 2.0, 1.0);
        let p2 = Point3::new(5.0, 6.0, 7.0);
        let expected = Vector3::new(-2.0, -4.0, -6.0);
        assert_eq!(p1 - p2, expected); 
    }

    #[test]
    fn point_minus_vector_is_point() {
        let p = Point3::new(3.0, 2.0, 1.0);
        let v = Vector3::new(5.0, 6.0, 7.0);
        let expected = Point3::new(-2.0, -4.0, -6.0);
        assert_eq!(p - v, expected);
    }
}