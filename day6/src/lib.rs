//! Advent of Code 2021, Day 6
//! https://adventofcode.com/2021/day/6
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!

use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let part_one = solve(&contents, 80);
    println!("Part one: {}", part_one);

    let part_two = solve(&contents, 256);
    println!("Part two: {}", part_two);

    Ok(())
}

fn solve(contents: &str, days: usize) -> u64 {
    let mut fish = contents
        .split(',')
        .fold([0u64; 9], |mut acc, val| {
            acc[val.parse::<usize>().unwrap()] += 1;
            acc
        });

    (0..days).for_each(|day| fish[(day + 7) % 9] += fish[day % 9]);

    fish.iter().sum()
}