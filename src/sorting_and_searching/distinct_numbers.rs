use std::collections::HashSet;
use std::iter::FromIterator;

#[allow(dead_code)]
fn distinct_numbers(nums: Vec<u64>) -> usize {
    HashSet::<u64>::from_iter(nums.into_iter()).len()
}

test! {
    test_1: distinct_numbers(vec![2, 3, 2, 2, 3]), 2,
    test_2: distinct_numbers(vec![]), 0,
    test_3: distinct_numbers(vec![1, 2, 3, 4]), 4,
}
