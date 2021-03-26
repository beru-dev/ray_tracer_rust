use std::ops::Add;
use crate::approx_equal::*;
use crate::point3::*;

pub mod vector3 {
    use super::Add;
    use super::point3::Point3;
    use super::approx_equal::*;

    #[derive(Debug, PartialEq)]
    pub struct Vector3(pub f64, pub f64, pub f64, pub f64);

    impl Vector3 {
        pub fn vector(x: f64, y: f64, z: f64) -> Vector3 {
            return Vector3(x, y, z, 0.0);
        }
    }

    impl Add for Vector3 {
        type Output = Vector3;

        fn add(self, other: Vector3) -> Vector3 {
            return Vector3::vector(
                &self.0 + other.0,
                &self.1 + other.1,
                &self.2 + other.2
            );
        }
    }

    impl Add<Point3> for Vector3 {
        type Output = Point3;

        fn add(self, other: Point3) -> Point3 {
            return Point3::point(
                &self.0 + other.0,
                &self.1 + other.1,
                &self.2 + other.2
            )
        }
    }
}