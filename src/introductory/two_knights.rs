#[allow(dead_code)]
fn two_knights(n: u64) -> Vec<i64> {
    let mut ans = vec![];
    for i in 1..=n {
        let i: i64 = i as i64;
        let possible = ((i * i) * (i * i - 1) / 2) - (4 * (i - 1) * (i - 2));
        ans.push(possible);
    }
    ans
}

test! {
    test_1: two_knights(2), vec![0, 6],
    test_2: two_knights(1), vec![0],
}
