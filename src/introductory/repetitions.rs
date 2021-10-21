#[allow(dead_code)]
fn repetitions(chars: Vec<char>) -> u64 {
    let mut curr = 1;
    let mut total = 1;
    let mut i = 0;

    for j in 1..chars.len() {
        let left = chars[i];
        let right = chars[j];
        curr = if left == right { curr + 1 } else { 1 };
        total = std::cmp::max(curr, total);
        i += 1;
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(repetitions("ATTCGGGA".chars().collect()), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(repetitions("ATTTTCGGGA".chars().collect()), 4);
    }

    #[test]
    fn test_3() {
        assert_eq!(repetitions("ATTCGGA".chars().collect()), 2);
    }
}
