use super::Matrix;
use crate::approx_equal::*;

#[derive(Debug, Copy, Clone)]
pub struct Matrix2 {
    matrix: [f64; 4]
}

impl Matrix2 {
    pub fn new(matrix: [f64; 4]) -> Matrix2 {
        Matrix2 {
            matrix
        }
    }

    pub fn determinant(&self) -> f64 {
        self.matrix[0] * self.matrix[3] - self.matrix[1] * self.matrix[2]
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        for (index, element) in self.matrix.iter().enumerate() {
            if !approx_equal::equal(*element, other.matrix[index]) {
                return false
            }
        }
        return true
    }
}

impl Matrix for Matrix2 {

    fn element(&self, row: usize, column: usize) -> f64 {
        self.matrix[Self::get_index(row, column)]
    }

    fn write(&mut self, row: usize, column: usize, new_val: f64) {
        self.matrix[Self::get_index(row, column)] = new_val;
    }

    fn get_index(row: usize, column: usize) -> usize {
        2 * row + column
    }

    fn transpose(&self) -> Self {
        let mut transposed = Matrix2::new([0.0; 4]);
        for row in 0..2 {
            for col in 0..2 {
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
    fn constructing_inspecting_matrix2() {
        let m = Matrix2::new([
            -3.0, 5.0,
            1.0, -2.0
        ]);

        assert_eq!(m.element(0, 0), -3.0);
        assert_eq!(m.element(0, 1), 5.0);
        assert_eq!(m.element(1, 0), 1.0);
        assert_eq!(m.element(1, 1), -2.0);
    }

    #[test]
    fn determinant_of_matrix2() {
        let a = Matrix2::new([
            1.0, 5.0,
            -3.0, 2.0
        ]);

        assert_eq!(a.determinant(), 17.0);
    }
}