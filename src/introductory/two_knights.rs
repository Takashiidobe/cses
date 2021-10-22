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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_knights(2), vec![0, 6]);
    }

    #[test]
    fn test_2() {
        assert_eq!(two_knights(1), vec![0]);
    }
}
