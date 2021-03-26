use std::ops::Add;
use crate::approx_equal::*;
use crate::vector3::*;

pub mod point3 {
    use super::Add;
    use super::vector3::Vector3;

    #[derive(Debug, PartialEq)]
    pub struct Point3(pub f64, pub f64, pub f64, pub f64);

    impl Point3 {
        pub fn point(x: f64, y: f64, z: f64) -> Point3 {
            return Point3(x, y, z, 1.0);
        }
    }

    impl Add<Vector3> for Point3 {
        type Output = Point3;

        fn add(self, other: Vector3) -> Point3 {
            return Point3::point(
                &self.0 + other.0,
                &self.1 + other.1,
                &self.2 + other.2
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::point3::*;
    use super::vector3::*;

    #[test]
    fn point_plus_vector_is_point() {
        let c = Point3::point(1.0, 2.0, 3.0) + Vector3::vector(2.0, 3.0, 4.0);
        assert_eq!(c.0, 3.0);
        assert_eq!(c.1, 5.0);
        assert_eq!(c.2, 7.0);
        assert_eq!(c.3, 1.0);
    }
}