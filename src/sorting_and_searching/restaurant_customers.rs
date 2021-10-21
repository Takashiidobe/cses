#[allow(dead_code)]
/// This problem gives an array of entering and leaving times for customers.
/// We want to find the maximum number of customers at any one time.
/// To do so, we first change the representation into something easier:
/// We iterate through the times, and duplicate it:
/// We create a new vector with a tuple of (enter_time, true) and
/// (leave_time, false). Then, we sort the vector based on times.
///
fn restaurant_customers(times: &[(u64, u64)]) -> u64 {
    let mut queue = vec![];
    for (enter_time, leave_time) in times {
        queue.push((enter_time, true));
        queue.push((leave_time, false));
    }

    queue.sort();

    let mut customers = 0;
    let mut max_customers_so_far = 0;

    for (_, r) in queue {
        match r {
            true => customers += 1,
            false => customers -= 1,
        }
        max_customers_so_far = u64::max(max_customers_so_far, customers);
    }

    max_customers_so_far
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            restaurant_customers(&[
                (1, 2),
                (3, 4),
                (5, 6),
                (7, 8),
                (9, 10),
                (11, 12),
                (13, 14),
                (15, 16),
                (17, 18),
                (19, 20)
            ]),
            1
        )
    }
}
