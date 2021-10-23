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

test! {
    test_1: repetitions("ATTCGGGA".chars().collect()), 3,
    test_2: repetitions("ATTTTCGGGA".chars().collect()), 4,
    test_3: repetitions("ATTCGGA".chars().collect()), 2,
}
