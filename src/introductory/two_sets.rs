#[allow(dead_code)]
fn two_sets(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let total: u64 = (1..=n).sum();
    if total % 2 != 0 {
        return None;
    }

    let mut left_total = 0;
    let mut left = vec![];

    let mut right_total = 0;
    let mut right = vec![];

    for i in (1..=n).rev() {
        if left_total < right_total {
            left_total += i;
            left.push(i);
        } else {
            right_total += i;
            right.push(i);
        }
    }

    Some((left, right))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(two_sets(1), None);
    }

    #[test]
    fn test_2() {
        assert_eq!(two_sets(2), None);
    }

    #[test]
    fn test_3() {
        assert_eq!(two_sets(3), Some((vec![2, 1], vec![3])));
    }
}
