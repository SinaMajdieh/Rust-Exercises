use std::{collections::HashMap};
fn main() {
    let numbers = vec![4, 1, 2, 2, 5, 6, 3, 2];
    let (median, mode) = get_median_and_mod(&numbers);
    println!("Median: {median} Mode: {mode}");
}


fn get_median_and_mod(numbers: &[i32]) -> (i32, i32) {
    let mut sorted = numbers.to_vec();
    sorted.sort_unstable();
    let median = sorted[sorted.len() / 2];

    let mut counts = HashMap::new();
    for num in numbers {
        *counts.entry(*num).or_insert(0) += 1;
    }
    let mode = counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap()
        .0;
    (median, mode)
}
