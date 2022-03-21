use super::Matrix;
use std::ops::{Mul};
use crate::approx_equal::*;
use crate::vector3::Vector3;
use crate::point3::Point3;
use crate::matrix::matrix3::Matrix3;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    pub matrix: [f64; 16]
}

impl Matrix4 {
    pub fn new(matrix: [f64; 16]) -> Self {
        Matrix4 {
            matrix
        }
    }

    pub fn identity() -> Self {
        Matrix4::new([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        ])
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut sub = Matrix3::new([0.0; 9]);
        for i in 0..4 {
            for j in 0..4 {
                if i == row || j == col { continue }
                sub.write(
                    if i < row { i } else { i - 1 },
                    if j < col { j } else { j - 1 },
                    self.element(i, j)
                );
            }
        }
        sub
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        for (index, element) in self.matrix.iter().enumerate() {
            if !approx_equal::equal(*element, other.matrix[index]) {
                return false
            }
        }
        return true
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut dot_product: Self = Self::new([0.0; 16]);

        for row in 0..4 {
            for col in 0..4 {
                let mut product: f64 = 0.0;
                for el in 0..4 {
                    product += self.element(row, el) * other.element(el, col);
                }
                dot_product.write(row, col, product);
            }
        }

        return dot_product;
    }
}

impl Mul<Point3> for Matrix4 {
    type Output = Point3;

    fn mul(self, other: Point3) -> Point3 {
        Point3::new(
            self.element(0, 0) * other.0 +
            self.element(0, 1) * other.1 +
            self.element(0, 2) * other.2 +
            self.element(0, 3),

            self.element(1, 0) * other.0 +
            self.element(1, 1) * other.1 +
            self.element(1, 2) * other.2 +
            self.element(1, 3),

            self.element(2, 0) * other.0 +
            self.element(2, 1) * other.1 +
            self.element(2, 2) * other.2 +
            self.element(2, 3)
        )
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.element(0, 0) * other.0 +
            self.element(0, 1) * other.1 +
            self.element(0, 2) * other.2,

            self.element(1, 0) * other.0 +
            self.element(1, 1) * other.1 +
            self.element(1, 2) * other.2,

            self.element(2, 0) * other.0 +
            self.element(2, 1) * other.1 +
            self.element(2, 2) * other.2
        )
    }
}

impl Matrix for Matrix4 {
    fn element(&self, row: usize, column: usize) -> f64 {
        self.matrix[Self::get_index(row, column)]
    }

    fn write(&mut self, row: usize, column: usize, new_val: f64) {
        self.matrix[Self::get_index(row, column)] = new_val;
    }

    fn get_index(row: usize, column: usize) -> usize {
        4 * row + column
    }

    fn transpose(&self) -> Self {
        let mut transposed = Matrix4::new([0.0; 16]);
        for row in 0..4 {
            for col in 0..4 {
                transposed.write(row, col, self.element(col, row));
            }
        }
        transposed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_inspecting_matrix4() {
        let m = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5
        ]);
        
        assert_eq!(m.element(0, 0), 1.0);
        assert_eq!(m.element(0, 3), 4.0);
        assert_eq!(m.element(1, 0), 5.5);
        assert_eq!(m.element(1, 2), 7.5);
        assert_eq!(m.element(2, 2), 11.0);
        assert_eq!(m.element(3, 0), 13.5);
        assert_eq!(m.element(3, 2), 15.5);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        let b = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        let b = Matrix4::new([
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0
        ]);
        let b = Matrix4::new([
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0
        ]);
        let expected = Matrix4::new([
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0
        ]);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn matrix_multiplied_by_point3() {
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        ]);
        let b = Point3::new(1.0, 2.0, 3.0);
        let expected = Point3::new(18.0, 24.0, 33.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn matrix_multiplied_by_vector3() {
        let a = Matrix4::new([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0
        ]);
        let b = Vector3::new(1.0, 2.0, 3.0);
        let expected = Vector3::new(14.0, 22.0, 32.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplying_matrix_by_identity() {
        let a = Matrix4::new([
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0
        ]);
        let identity = Matrix4::identity();

        assert_eq!(a * identity, a);
    }

    #[test]
    fn multiplying_identity_by_tuple() {
        let a = Point3::new(1.0, 2.0, 3.0);
        let identity = Matrix4::identity();

        assert_eq!(identity * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix4::new([
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0
        ]);
        let expected = Matrix4::new([
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0
        ]);

        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn transposing_identity_matrix() {
        let id = Matrix4::identity();

        assert_eq!(id.transpose(), id);
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix4::new([
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0
        ]);
        let expected = 
        Matrix3::new([
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0
        ]);

        assert_eq!(a.submatrix(2, 1), expected);
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let a = Matrix4::new([
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0
        ]);

        
    }
}