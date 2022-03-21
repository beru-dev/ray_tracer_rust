pub mod matrix4;
pub mod matrix3;
pub mod matrix2;

pub trait Matrix {
    fn element(&self, row: usize, column: usize) -> f64;

    fn write(&mut self, row: usize, column: usize, new_val: f64) -> ();

    fn get_index(row: usize, column: usize) -> usize;

    fn transpose(&self) -> Self;

    // fn submatrix<T>(&self, row: usize, col: usize) -> T;
}