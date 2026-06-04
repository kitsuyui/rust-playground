pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn multiply_sum(x: i64, y: i64, factor: i64) -> String {
    ((x + y) * factor).to_string()
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
        assert_eq!(multiply_sum(1, 2, 3), "9");
    }
}
