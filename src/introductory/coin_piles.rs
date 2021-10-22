#[allow(dead_code)]
fn coin_piles(piles: &mut [(u64, u64)]) -> Vec<bool> {
    // if the sum of the piles isn't divisible by 3, its not possible.
    // otherwise, if one pile is > 2/3 of the pile, not possible either.
    let mut v = vec![];
    for (left, right) in piles.into_iter() {
        if *left + *right % 3 != 0 {
            v.push(false);
        } else if *left * 2 == *right || *right * 2 == *left {
            v.push(true);
        } else if *left * 2 < *right || *right * 2 < *left {
            v.push(false);
        } else {
            v.push(true);
        }
    }

    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(coin_piles(&mut [(11, 4)]), vec![false]);
    }
}
