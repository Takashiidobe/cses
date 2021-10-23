use std::collections::HashSet;

#[allow(dead_code)]
fn ferris_wheel(children: &mut [u64], weight: u64) -> u64 {
    // sort the children
    children.sort();
    // create a left and right pointer, and increment
    let mut i = 0;
    let mut j = children.len() - 1;
    let mut count = 0;

    // keep a set of visited locations
    let mut set = HashSet::new();

    // keep current weight
    let mut current_weight = 0;
    while i < j {
        // take the ith child.
        current_weight += children[i];
        i += 1;
        set.insert(i);
        // try to take as many children as you can.
        loop {
            if i == j {
                break;
            }
            if current_weight + children[j] <= weight {
                current_weight += children[j];
                set.insert(j);
            }
            j -= 1;
        }
        count += 1;
        current_weight = 0;
    }
    count + ((children.len() - set.len()) as u64)
}

test! {
    test_1: ferris_wheel(&mut [7, 2, 3, 9], 10), 3,
    // test_2: ferris_wheel(&mut [6, 10, 7, 6, 9, 10, 7, 7], 22), 4,
}
