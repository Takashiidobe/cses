#[allow(dead_code)]
fn missing_number(n: u64, nums: Vec<u64>) -> u64 {
    let total: u64 = (1..=n).sum();
    let num_total: u64 = nums.into_iter().sum();
    total - num_total
}

test! {
    test_1: missing_number(5, vec![2, 3, 1, 5]), 4,
    test_2: missing_number(3, vec![1, 2]), 3,
    test_3: missing_number(6, vec![1, 5, 4, 3, 2]), 6,
}
