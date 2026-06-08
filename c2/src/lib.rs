/// Adds two unsigned integers.
///
/// # Examples
///
/// ```
/// use kitsuyui_rust_playground_lib::add;
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Computes `(a + b) * c` and returns the result as a decimal string.
///
/// # Examples
///
/// ```
/// use kitsuyui_rust_playground_lib::my_calc;
/// assert_eq!(my_calc(1, 2, 3), "9");
/// ```
pub fn my_calc(a: i64, b: i64, c: i64) -> String {
    ((a + b) * c).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_my_calc() {
        assert_eq!(my_calc(1, 2, 3), "9");
    }
}
