use std::collections::HashSet;
use std::iter::FromIterator;

#[allow(dead_code)]
fn distinct_numbers(nums: Vec<u64>) -> usize {
    HashSet::<u64>::from_iter(nums.into_iter()).len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(distinct_numbers(vec![2, 3, 2, 2, 3]), 2);
    }

    #[test]
    fn test_2() {
        assert_eq!(distinct_numbers(vec![]), 0);
    }

    #[test]
    fn test_3() {
        assert_eq!(distinct_numbers(vec![1, 2, 3, 4]), 4);
    }
}
