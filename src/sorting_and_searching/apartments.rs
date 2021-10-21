#[allow(dead_code)]
fn apartments(mut people: Vec<i64>, mut apartments: Vec<i64>, k: i64) -> u64 {
    people.sort();
    apartments.sort();

    let mut ans = 0;

    let mut i = 0;
    let mut j = 0;

    while i < people.len() && j < apartments.len() {
        if i64::abs(people[i] - apartments[j]) <= k {
            ans += 1;
            i += 1;
            j += 1;
        } else if people[i] < apartments[j] {
            i += 1;
        } else {
            j += 1;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            apartments(
                vec![37, 62, 56, 69, 34, 46, 10, 86, 16, 49],
                vec![50, 95, 47, 43, 9, 62, 83, 71, 71, 7],
                0
            ),
            1
        )
    }
}
