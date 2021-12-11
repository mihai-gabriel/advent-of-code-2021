//! Advent of Code 2021, Day 10
//! https://adventofcode.com/2021/day/10
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!
//! Notes:
//! I'm using a stack to match the order
//! The next closing bracket should match the last opening bracket

use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let part_one = solve_part_one(&contents);
    println!("part one: {}", part_one);

    let part_two = solve_part_two(&contents);
    println!("part two: {}", part_two);

    Ok(())
}

fn bracket_match(bracket: char) -> char {
    match bracket {
        // opening for
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        // closing for
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!()
    }
}

fn points_for(bracket: char) -> u64 {
    match bracket {
        // for illegal chars
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        // for completion
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!()
    }
}

fn solve_part_one(contents: &str) -> u64 {
    let mut illegal_chars = vec![];

    for line in contents.lines() {
        let mut syntax = Vec::new();
        for bracket in line.chars() {
            match &bracket {
                '(' | '[' | '{' | '<' => syntax.push(bracket),
                ')' | ']' | '}' | '>' => {
                    let opening_bracket = syntax.pop().unwrap();
                    if opening_bracket != bracket_match(bracket) {
                        illegal_chars.push(points_for(bracket));
                        break;
                    }
                }
                _ => unreachable!()
            }
        }
    }

    illegal_chars.iter().sum()
}

fn solve_part_two(contents: &str) -> u64 {
    let mut points = vec![];
    'next_line: for line in contents.lines() {
        let mut syntax = Vec::new();
        for bracket in line.chars() {
            match &bracket {
                '(' | '[' | '{' | '<' => syntax.push(bracket),
                ')' | ']' | '}' | '>' => {
                    let opening_bracket = syntax.pop().unwrap();
                    if opening_bracket != bracket_match(bracket) {
                        continue 'next_line; // filter corrupted lines
                    }
                }
                _ => unreachable!()
            }
        }

        let mut score = 0;
        while let Some(opening) = syntax.pop() {
            score = score * 5 + points_for(opening);
        }
        points.push(score);
    }

    points.sort();
    points[points.len() / 2]
}