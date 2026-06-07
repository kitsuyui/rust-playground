pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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
