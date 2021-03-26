pub mod approx_equal {
    pub fn equal(f1: f64, f2: f64) -> bool {
        let epsilon = 0.00001;
        if (f1 - f2).abs() < epsilon {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::approx_equal::equal;

    #[test]
    fn small_differences_are_equal() {
        assert!(equal(1.0, 1.000001));
    }
}