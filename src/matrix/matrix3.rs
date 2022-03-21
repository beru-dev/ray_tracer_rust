use super::Matrix;
use crate::approx_equal::*;
use crate::matrix::matrix2::Matrix2;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3 {
    matrix: [f64; 9]
}

impl Matrix3 {
    pub fn new(matrix: [f64; 9]) -> Matrix3 {
        Matrix3 {
            matrix
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut sub = Matrix2::new([0.0; 4]);
        for i in 0..3 {
            for j in 0..3 {
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

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 == 0 {
            return self.minor(row, col);
        } else {
            return -self.minor(row, col);
        }
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        for (index, element) in self.matrix.iter().enumerate() {
            if !approx_equal::equal(*element, other.matrix[index]) {
                return false
            }
        }
        return true
    }
}

impl Matrix for Matrix3 {
    fn element(&self, row: usize, column: usize) -> f64 {
        self.matrix[Self::get_index(row, column)]
    }

    fn write(&mut self, row: usize, column: usize, new_val: f64) {
        self.matrix[Self::get_index(row, column)] = new_val;
    }

    fn get_index(row: usize, column: usize) -> usize {
        3 * row + column
    }

    fn transpose(&self) -> Self {
        let mut transposed = Matrix3::new([0.0; 9]);
        for row in 0..3 {
            for col in 0..3 {
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
    fn constructing_inspecting_matrix3() {
        let m = Matrix3::new([
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0
        ]);

        assert_eq!(m.element(0, 0), -3.0);
        assert_eq!(m.element(1, 1), -2.0);
        assert_eq!(m.element(2, 2), 1.0);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix3::new([
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0
        ]);
        let expected = Matrix2::new([
            -3.0, 2.0,
            0.0, 6.0
        ]);

        assert_eq!(a.submatrix(0, 2), expected);
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let a = Matrix3::new([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ]);

        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let a = Matrix3::new([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0
        ]);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let a = Matrix3::new([
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0
        ]);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }
}