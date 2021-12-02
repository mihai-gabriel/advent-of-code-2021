//! Advent of Code 2021, Day 2
//! https://adventofcode.com/2021/day/2
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!
//! Notes:
//! Lack of comments because the code is pretty self-explanatory

mod types;

use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use types::{Directions, Position};


pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let part_one = solve(&contents, solve_part_one);
    println!("Part one: {}", part_one);

    let part_two = solve(&contents, solve_part_two);
    println!("Part two: {}", part_two);

    Ok(())
}

/// Working with types instead of strings
fn parse_direction(line: &str) -> Directions {
    let mut split = line.split_whitespace();
    let direction = split.next().unwrap();
    let value: i32 = split.next().unwrap().parse().unwrap();

    if let Ordering::Equal = direction.cmp("forward") {
        return Directions::Forward(value);
    } else if let Ordering::Equal = direction.cmp("up") {
        return Directions::Up(value);
    } else if let Ordering::Equal = direction.cmp("down") {
        return Directions::Down(value);
    }

    panic!("Found invalid direction!");
}

fn solve(contents: &String, solver: fn(Directions, &mut Position)) -> i32 {
    let mut submarine_pos = Position::new();

    for line in contents.lines() {
        let direction = parse_direction(line);
        solver(direction, &mut submarine_pos);
    }
    submarine_pos.final_position()
}

fn solve_part_one(direction: Directions, position: &mut Position) {
    match direction {
        Directions::Up(value) => position.add_depth(-value),
        Directions::Down(value) => position.add_depth(value),
        Directions::Forward(value) => position.add_horizontal(value)
    }
}

fn solve_part_two(direction: Directions, position: &mut Position) {
    match direction {
        Directions::Up(value) => position.add_aim(-value),
        Directions::Down(value) => position.add_aim(value),
        Directions::Forward(value) => {
            position.add_horizontal(value);
            position.add_depth(position.get_aim() * value);
        }
    }
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    #[should_panic]
    fn test_invalid_direction() {
        parse_direction("backward 30");
    }

    #[test]
    #[should_panic]
    fn test_no_value_direction() {
        parse_direction("forward");
    }

    #[test]
    #[should_panic]
    fn test_non_integer_value() {
        parse_direction("forward abcd");
    }

    #[test]
    #[should_panic]
    fn test_empty_line() {
        parse_direction("");
    }

    #[test]
    fn test_parse_direction() {
        let direction = parse_direction("up 20");
        if let Directions::Up(value) = direction {
            assert_eq!(value, 20);
        }

        let direction = parse_direction("down 15");
        if let Directions::Down(value) = direction {
            assert_eq!(value, 15);
        }

        let direction = parse_direction("forward 50");
        if let Directions::Forward(value) = direction {
            assert_eq!(value, 50);
        }
    }

    #[test]
    fn test_part_one() {
        let mut submarine_position = Position::new();

        let direction = Directions::Up(10);
        solve_part_one(direction, &mut submarine_position);

        let direction = Directions::Down(40);
        solve_part_one(direction, &mut submarine_position);

        let direction = Directions::Forward(10);
        solve_part_one(direction, &mut submarine_position);

        let direction = Directions::Forward(5);
        solve_part_one(direction, &mut submarine_position);

        assert_eq!(submarine_position.get_depth(), -10 + 40);
        assert_eq!(submarine_position.get_horizontal(), 10 + 5);
    }

    #[test]
    fn test_part_two() {
        let mut submarine_position = Position::new();

        let direction = Directions::Up(10);
        solve_part_two(direction, &mut submarine_position);

        let direction = Directions::Down(40);
        solve_part_two(direction, &mut submarine_position);

        let direction = Directions::Forward(10);
        solve_part_two(direction, &mut submarine_position);

        let direction = Directions::Forward(5);
        solve_part_two(direction, &mut submarine_position);

        assert_eq!(submarine_position.get_aim(), -10 + 40);
        assert_eq!(submarine_position.get_depth(), 10 * 30 + 5 * 30);
        assert_eq!(submarine_position.get_horizontal(), 10 + 5);
    }
}
