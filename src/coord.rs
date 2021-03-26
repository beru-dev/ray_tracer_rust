use crate::approx_equal::*;

pub mod coord {
    pub struct Coord(pub f64, pub f64, pub f64, pub f64);

    impl Coord {
        pub fn point(x: f64, y: f64, z: f64) -> Coord {
            return Coord(x, y, z, 1.0);
        }

        pub fn vector(x: f64, y: f64, z: f64) -> Coord {
            return Coord(x, y, z, 0.0);
        }

        pub fn compare(c1: Coord, c2: Coord) -> bool {
            use super::approx_equal::equal;
            equal(c1.0, c2.0) &&
            equal(c1.1, c2.1) &&
            equal(c1.2, c2.2) &&
            equal(c1.3, c2.3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::coord::*;

    #[test]
    fn tuple_with_w1_is_point() {
        let a = Coord(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.0, 4.3);
        assert_eq!(a.1, -4.2);
        assert_eq!(a.2, 3.1);
        assert_eq!(a.3, 1.0);
        assert_ne!(a.3, 0.0);
    }

    #[test]
    fn tuple_with_w0_is_vector() {
        let a = Coord(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.0, 4.3);
        assert_eq!(a.1, -4.2);
        assert_eq!(a.2, 3.1);
        assert_eq!(a.3, 0.0);
        assert_ne!(a.3, 1.0);
    }

    #[test]
    fn point_creates_w1_tuples() {
        let p = Coord::point(4.0, -4.0, 3.0);
        let c = Coord(4.0, -4.0, 3.0, 1.0);
        assert!(Coord::compare(p, c));
    }

    #[test]
    fn vector_creates_w2_tuples() {
        let v = Coord::vector(4.0, -4.0, 3.0);
        let c = Coord(4.0, -4.0, 3.0, 0.0);
        assert!(Coord::compare(v, c));
    }
}