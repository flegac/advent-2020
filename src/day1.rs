use std::cmp::Ordering;

use itertools::{Itertools, sorted};

use crate::utils::read_lines;

const TARGET: i32 = 2020;


fn parse_file1(filename: &str) -> Vec<i32> {
    let mut all = vec![];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                all.push(value.parse::<i32>().unwrap());
            }
        }
    }
    return all;
}

pub fn solve1(n: usize) -> Option<i32> {
    parse_file1("src/day1.txt")
        .into_iter()
        .combinations(n)
        .filter(|vec| vec.iter().sum::<i32>() == TARGET)
        .find(|vec| vec.iter().sum::<i32>() == TARGET)
        .map(|vec| vec.iter().product::<i32>())
}

fn solve2(n: usize) -> Option<i32> {
    parse_file1("src/day1.txt")
        .iter()
        .copied()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == TARGET)
        .map(|(a, b, c)| a * b * c)
}

pub fn solve_fast(n: usize) -> Option<i32> {
    let items = sorted(parse_file1("src/day1.txt")).collect_vec();
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


pub fn day1_benchmark() {
    println!("{} {}",
             solve1(2).expect("No solution"),
             solve1(3).expect("No solution"));
    timeit!({
        solve1(2);
        solve1(3);
    });
    // 1 loops: 557.8177 ms

    println!("{} {}",
             solve_fast(2).expect("No solution"),
             solve_fast(3).expect("No solution")
    );
    timeit!({
         solve_fast(2);
         solve_fast(3);
    });
    // 1000 loops: 1.0746479 ms
}