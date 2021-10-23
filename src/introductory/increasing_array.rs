pub fn increasing_array(mut nums: Vec<u64>) -> u64 {
    let mut total = 0;

    let mut i = 0;

    for j in 1..nums.len() {
        if nums[i] > nums[j] {
            total += nums[i] - nums[j];
            nums[j] = nums[i];
        }

        i += 1;
    }

    total
}

test! {
    test_1: increasing_array(vec![1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 8999999991,
    test_2: increasing_array(vec![3, 2, 5, 1, 7]), 5,
    test_3: increasing_array(vec![6, 10, 4, 10, 2, 8, 9, 2, 7, 7]), 31,
}
