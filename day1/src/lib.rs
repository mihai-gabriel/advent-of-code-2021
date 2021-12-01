//! Advent of Code 2021, Day 1
//! https://adventofcode.com/2021/day/1
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!
use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let part_one = solve_part_one(convert_to_i32(&contents));
    println!("Part one: {}", part_one);

    let part_two = solve_part_two(convert_to_i32(&contents));
    println!("Part two: {}", part_two);

    Ok(())
}

/// Utility for converting from string lines to vector of integers
fn convert_to_i32(contents: &String) -> Vec<i32> {
    contents
        .lines()
        .map(|nr_str| nr_str.parse().unwrap())
        .collect()
}

/// Compares two contiguous elements from a Vector of Integers
/// e.g. vec![1, 2, 3, 2] -> [1, 2], [2, 3], [3, 2]
/// Returns the number of times the next value is increasing
/// e.g. increasing for [1, 2], [2, 3], but not for [3, 2] -> 2 times
fn solve_part_one(lines: Vec<i32>) -> i32 {
    lines.windows(2).filter(|nums| nums[0] < nums[1]).count() as i32
}

/// Add three contiguous elements from a Vector of Integers
/// Returns the resulting Vector
fn solve_part_two(lines: Vec<i32>) -> i32 {
    solve_part_one(
        lines
            .windows(3)
            .map(|nums| nums[0] + nums[1] + nums[2])
            .collect(),
    )
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_part_one() {
        let numbers = vec![1, 5, 4, 2, 3, 5];
        assert_eq!(solve_part_one(numbers), 3);

        let numbers = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(solve_part_one(numbers), 0);

        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(solve_part_one(numbers), 8);
    }

    #[test]
    fn test_part_two() {
        let numbers = vec![1, 5, 4, 2, 3, 5];
        assert_eq!(solve_part_two(numbers), 2);

        let numbers = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(solve_part_two(numbers), 0);

        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(solve_part_two(numbers), 6);
    }
}
