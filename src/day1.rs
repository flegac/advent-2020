use std::cmp::Ordering;

use itertools::{Itertools, sorted};

use crate::utils::read_lines;

type Int = u32;
type Int2 = u64;

// const FILENAME: &str = "src/day1.txt";
// const TARGET: Int = 2020;

const FILENAME: &str = "src/bad_case.txt";
const TARGET: Int = 300006;


fn parse_file1(filename: &str) -> Vec<Int> {
    let mut all = vec![];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(value) = line {
                all.push(value.parse::<Int>().unwrap());
            }
        }
    }
    return all;
}

pub fn solve1(n: usize) -> Option<Int2> {
    parse_file1(FILENAME)
        .into_iter()
        .combinations(n)
        .filter(|vec| vec.iter().sum::<Int>() == TARGET)
        .find(|vec| vec.iter().sum::<Int>() == TARGET)
        .map(|vec| vec.iter().product::<Int>() as Int2)
}

fn solve2(n: usize) -> Option<Int2> {
    parse_file1(FILENAME)
        .iter()
        .copied()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == TARGET)
        .map(|(a, b, c)| (a * b * c) as Int2)
}

pub fn solve_fast(n: usize) -> Option<Int2> {
    let items = sorted(parse_file1(FILENAME)).collect_vec();
    solve(&items, TARGET, n)
}

fn solve(items: &[Int], target: Int, n: usize) -> Option<Int2> {
    if n == 1 {
        return search(items, target);
    }
    for i in 0..items.len() {
        if items[i] > target {
            break
        }
        if let Some(value) = solve(&items[i + 1..], target - items[i], n - 1) {
            return Some(value * items[i] as Int2);
        }
    }
    None
}

fn search(items: &[Int], target: Int) -> Option<Int2> {
    if items.is_empty() || target < items[0] || target > items[items.len() - 1] {
        return None;
    }
    let j = items.len() / 2;
    match items[j].cmp(&target) {
        Ordering::Equal => { Some(target as Int2) }
        Ordering::Greater => { search(&items[0..j], target) }
        Ordering::Less => { search(&items[j + 1..], target) }
    }
}


pub fn day1_benchmark() {
    // println!("{} {}",
    //          solve1(2).expect("No solution"),
    //          solve1(3).expect("No solution"));
    // timeit!({
    //     solve1(2);
    //     solve1(3);
    // });
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