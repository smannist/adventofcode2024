use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in stdin.lock().lines().filter_map(Result::ok) {
        let mut iter = line.split_whitespace();
        if let (Some(l), Some(r)) = (iter.next(), iter.next()) {
            if let (Ok(l), Ok(r)) = (l.parse::<u32>(), r.parse::<u32>()) {
                left.push(l);
                right.push(r);
            }
        }
    }

    left.sort_unstable();
    right.sort_unstable();

    let total_distance: u32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("Total Distance = {}", total_distance);

    let right_counts: HashMap<u32, u32> = right.into_iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let similarity_score: u32 = left
        .into_iter()
        .map(|num| num * right_counts.get(&num).unwrap_or(&0))
        .sum();

    println!("Similarity Score = {}", similarity_score);
}
