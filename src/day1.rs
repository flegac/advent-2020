use std::cmp::Ordering;

use itertools::{Itertools, sorted};

use crate::utils;

const TARGET: i32 = 2020;

pub fn solve1(n: usize) -> Option<i32> {
    utils::parse_file("src/day1.txt")
        .into_iter()
        .combinations(n)
        .filter(|vec| vec.iter().sum::<i32>() == TARGET)
        .find(|vec| vec.iter().sum::<i32>() == TARGET)
        .map(|vec| vec.iter().product::<i32>())
}

fn solve2(n: usize) -> Option<i32> {
    utils::parse_file("src/day1.txt")
        .iter()
        .copied()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == TARGET)
        .map(|(a, b, c)| a * b * c)
}

pub fn solve_fast(n: usize) -> Option<i32> {
    let items = sorted(utils::parse_file("src/day1.txt")).collect_vec();
    solve(&items, TARGET, n)
}

fn solve(items: &[i32], target: i32, n: usize) -> Option<i32> {
    if n == 1 {
        return search(items, target);
    }
    for i in 0..items.len() {
        if let Some(value) = solve(&items[i + 1..], target - items[i], n - 1) {
            return Some(items[i] * value);
        }
    }
    None
}

fn search(items: &[i32], target: i32) -> Option<i32> {
    if items.is_empty() || target < items[0] || target > items[items.len() - 1] {
        return None;
    }
    let j = items.len() / 2;
    match items[j].cmp(&target) {
        Ordering::Equal => { Some(target) }
        Ordering::Greater => { search(&items[0..j], target) }
        Ordering::Less => { search(&items[j + 1..], target) }
    }
}

