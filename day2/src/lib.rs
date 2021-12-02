//! Advent of Code 2021, Day 2
//! https://adventofcode.com/2021/day/2
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!
//! Notes:
//! Lack of comments because the code is pretty self-explanatory

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

struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

fn solve_part_one(contents: &str) -> i32 {
    let mut position = Position::new();

    for line in contents.lines() {
        let (direction, value) = line.split_once(" ").unwrap();
        let value: i32 = value.parse().unwrap();

        match direction {
            "up" => position.depth -= value,
            "down" => position.depth += value,
            "forward" => position.horizontal += value,
            _ => unreachable!()
        }
    }

    position.depth * position.horizontal
}

fn solve_part_two(contents: &str) -> i32 {
    let mut position = Position::new();

    for line in contents.lines() {
        let (direction, value) = line.split_once(" ").unwrap();
        let value: i32 = value.parse().unwrap();

        match direction {
            "up" => position.aim -= value,
            "down" => position.aim += value,
            "forward" => {
                position.horizontal += value;
                position.depth += position.aim * value;
            },
            _ => unreachable!()
        }
    }

    position.depth * position.horizontal
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test_part_one() {
        let contents = "up 10\ndown 40\nforward 10\nforward 5";
        assert_eq!(solve_part_one(&contents), 30 * 15);
    }

    #[test]
    fn test_part_two() {
        let contents = "up 10\ndown 40\nforward 10\nforward 5";
        assert_eq!(solve_part_two(&contents), 450 * 15);
    }
}
