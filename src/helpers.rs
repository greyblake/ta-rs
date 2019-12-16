/// Returns the largest of 3 given numbers.
pub fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max3() {
        assert_eq!(max3(3.0, 2.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 3.0, 1.0), 3.0);
        assert_eq!(max3(2.0, 1.0, 3.0), 3.0);
    }
}
