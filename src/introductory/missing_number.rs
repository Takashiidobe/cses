#[allow(dead_code)]
fn missing_number(n: u64, nums: Vec<u64>) -> u64 {
    let total: u64 = (1..=n).sum();
    let num_total: u64 = nums.into_iter().sum();
    total - num_total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(missing_number(5, vec![2, 3, 1, 5]), 4);
    }
    #[test]
    fn test_2() {
        assert_eq!(missing_number(3, vec![1, 2]), 3);
    }

    #[test]
    fn test_3() {
        assert_eq!(missing_number(6, vec![1, 5, 4, 3, 2]), 6);
    }
}
