//! Advent of Code 2021, Day 8
//! https://adventofcode.com/2021/day/8
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!

use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let part_one = solve_part_one(&contents);
    println!("Part one: {}", part_one);

    let part_two = solve_part_two(&contents);
    println!("Part two: {}", part_two);

    Ok(())
}

fn solve_part_one(contents: &str) -> i32 {
    let mut result = 0;
    for line in contents.lines() {
        let (_, output) = line.split_once('|').unwrap();
        result += output
            .split_whitespace()
            .fold(0i32, |acc, val| if [2, 3, 4, 7].contains(&val.len()) { acc + 1 } else { acc });
    }

    result
}

fn solve_part_two(contents: &str) -> i32 {
    let mut result = 0;
    for line in contents.lines() {
        let (patterns, output) = line.split_once('|').unwrap();

        let digits: Vec<&str> = patterns.split_whitespace().collect();
        let d_one = digits.iter().find(|x| x.len() == 2).unwrap();
        let d_four = digits.iter().find(|x| x.len() == 4).unwrap();

        let mut digit_output = 0;
        for digit in output.split_whitespace() {
            digit_output = digit_output * 10 + match digit.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                other => {
                    let (diff_one, diff_four) = (
                        digit.chars().filter(|x| !d_one.contains(&x.to_string())).count(),
                        digit.chars().filter(|x| !d_four.contains(&x.to_string())).count()
                    );

                    match (other, diff_one, diff_four) {
                        (6, 4, 3) => 0,
                        (5, 4, 3) => 2,
                        (5, 3, 2) => 3,
                        (5, 4, 2) => 5,
                        (6, 5, 3) => 6,
                        (6, 4, 2) => 9,
                        _ => unreachable!()
                    }
                }
            }
        }
        result += digit_output;
    }
    result
}
