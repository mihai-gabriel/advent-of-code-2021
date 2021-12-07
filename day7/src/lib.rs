//! Advent of Code 2021, Day 7
//! https://adventofcode.com/2021/day/7
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!

use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let data: Vec<i64> = contents.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

    let part_one = solve_part_one(&data);
    println!("Part one: {}", part_one);

    let part_two = solve_part_two(&data);
    println!("Part two: {}", part_two);

    Ok(())
}

fn solve_part_one(data: &Vec<i64>) -> i64 {
    let mut result = vec![];
    for i in 1..=*data.iter().max().unwrap() {
        result.push(data.iter().fold(0i64, |acc, val| acc + i64::abs(val - i)));
    }

    *result.iter().min().unwrap()
}

fn gauss(nr: i64) -> i64 {
    nr * (nr + 1)/2
}

fn solve_part_two(data: &Vec<i64>) -> i64 {
    let mut result = vec![];
    for i in 1..=*data.iter().max().unwrap() {
        result.push(data.iter().fold(0i64, |acc, val| acc + gauss(i64::abs(val - i))));
    }

    *result.iter().min().unwrap()
}