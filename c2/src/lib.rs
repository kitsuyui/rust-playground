use std::fmt;

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

/// Error returned when `multiply_sum` overflows the `i64` range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MultiplySumOverflow;

impl fmt::Display for MultiplySumOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(x + y) * factor overflowed the i64 range")
    }
}

impl std::error::Error for MultiplySumOverflow {}

/// Computes `(x + y) * factor` and returns the result as a decimal string.
///
/// # Examples
///
/// ```
/// use kitsuyui_rust_playground_lib::multiply_sum;
/// assert_eq!(multiply_sum(1, 2, 3).unwrap(), "9");
/// ```
pub fn multiply_sum(x: i64, y: i64, factor: i64) -> Result<String, MultiplySumOverflow> {
    let sum = x.checked_add(y).ok_or(MultiplySumOverflow)?;
    let product = sum.checked_mul(factor).ok_or(MultiplySumOverflow)?;
    Ok(product.to_string())
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
    fn test_multiply_sum() {
        assert_eq!(multiply_sum(1, 2, 3), Ok("9".to_string()));
    }

    #[test]
    fn multiply_sum_reports_add_overflow() {
        assert_eq!(multiply_sum(i64::MAX, 1, 1), Err(MultiplySumOverflow));
    }

    #[test]
    fn multiply_sum_reports_multiply_overflow() {
        assert_eq!(multiply_sum(i64::MAX, 0, 2), Err(MultiplySumOverflow));
    }
}
