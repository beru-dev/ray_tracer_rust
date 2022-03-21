use std::ops::{Add, Sub, Neg, Mul, Div};
use crate::approx_equal::*;
use crate::point3::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector3(pub f64, pub f64, pub f64, pub f64);

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3(x, y, z, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        Vector3::new(
            self.0 / self.magnitude(),
            self.1 / self.magnitude(),
            self.2 / self.magnitude()
        )
    }

    pub fn cross(&self, other: &Vector3) ->  Vector3 {
        Vector3::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Add<Point3> for Vector3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        use approx_equal::equal;
        equal(self.0, other.0) &&
        equal(self.1, other.1) &&
        equal(self.2, other.2)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.0, -self.1, -self.2)
    }
}

impl Mul for Vector3 {
    type Output = f64;

    fn mul(self, other: Vector3) -> f64 {
        self.0 * other.0 +
        self.1 * other.1 +
        self.2 * other.2
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3::new(
            self.0 * other,
            self.1 * other,
            self.2 * other
        )
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3::new(
            self.0 / other,
            self.1 / other,
            self.2 / other
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_plus_point_is_point() {
        let p = Vector3::new(-2.0, 3.0, 1.0) + Point3::new(3.0, -2.0, 5.0);
        assert_eq!(p, Point3::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn vector_plus_vector_is_vector() {
        let p = Vector3::new(-2.0, 3.0, 1.0) + Vector3::new(3.0, -2.0, 5.0);
        assert_eq!(p, Vector3::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn vector_minus_vector_is_vector() {
        let v1 = Vector3::new(3.0, 2.0, 1.0);
        let v2 = Vector3::new(5.0, 6.0, 7.0);
        let expected = Vector3::new(-2.0, -4.0, -6.0);
        assert_eq!(v1 - v2, expected);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Vector3::new(0.0, 0.0, 0.0);
        let v = Vector3::new(1.0, -2.0, 3.0);
        let expected = Vector3::new(-1.0, 2.0, -3.0);

        assert_eq!(zero - v, expected);
    }

    #[test]
    fn negating_a_tuple() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        let expected = Vector3::new(-1.0, 2.0, -3.0);

        assert_eq!(-v, expected);
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Vector3::new(1.0, -2.0, 3.0) * 3.5;
        let expected = Vector3::new(3.5, -7.0, 10.5);

        assert_eq!(a, expected);
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Vector3::new(1.0, -2.0, 3.0) * 0.5;
        let expected = Vector3::new(0.5, -1.0, 1.5);

        assert_eq!(a, expected);
    }

    #[test]
    fn dividing_tuple_by_scalar() {
        let a = Vector3::new(1.0, -2.0, 3.0) / 2.0;
        let expected = Vector3::new(0.5, -1.0, 1.5);

        assert_eq!(a, expected);
    }

    #[test]
    fn vector_magnitude_x() {
        let v = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude_y() {
        let v = Vector3::new(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude_z() {
        let v = Vector3::new(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn vector_magnitude_123() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let f: f64 = 14.0;

        assert_eq!(v.magnitude(), f.sqrt());
    }

    #[test]
    fn vector_magnitude_neg_123() {
        let v = Vector3::new(-1.0, -2.0, -3.0);
        let f: f64 = 14.0;

        assert_eq!(v.magnitude(), f.sqrt());
    }

    #[test]
    fn normalizing_v400_gives100() {
        let v = Vector3::new(4.0, 0.0, 0.0);
        let expected = Vector3::new(1.0, 0.0, 0.0);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn normalizing_v123() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let expected = Vector3::new(0.26726, 0.53452, 0.80178);

        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let norm = v.normalize();

        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_vectors() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(2.0, 3.0, 4.0);

        assert_eq!(a * b, 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Vector3::new(1.0, 2.0, 3.0);
        let b = Vector3::new(2.0, 3.0, 4.0);
        let ab = Vector3::new(-1.0, 2.0, -1.0);
        let ba = Vector3::new(1.0, -2.0, 1.0);

        assert_eq!(a.cross(&b), ab);
        assert_eq!(b.cross(&a), ba);
    }
}