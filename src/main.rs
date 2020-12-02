#[macro_use]
extern crate timeit;

use crate::day1::{solve1, solve_fast};

mod day1;
mod utils;


fn main() {
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