//! Advent of Code 2021, Day 5
//! https://adventofcode.com/2021/day/5
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!

use std::collections::HashMap;
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

struct CoordPair {
    from: (i32, i32),
    to: (i32, i32),
}

impl CoordPair {
    fn iter(&self) -> CoordIterator {
        CoordIterator::new(self.from, self.to)
    }
}

struct CoordIterator {
    next: (i32, i32),
    end: (i32, i32),
    step_x: i32,
    step_y: i32,
}

impl CoordIterator {
    fn new(start: (i32, i32), end: (i32, i32)) -> Self {
        let step_x = (end.0 - start.0).signum();
        let step_y = (end.1 - start.1).signum();
        CoordIterator {
            next: start,
            end,
            step_x,
            step_y,
        }
    }
}

impl Iterator for CoordIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self>::Item> {
        let curr = self.next;
        self.next = (
            self.next.0 + self.step_x,
            self.next.1 + self.step_y,
        );
        if (self.step_x > 0 && curr.0 > self.end.0) || (self.step_x < 0 && curr.0 < self.end.0) {
            return None;
        }
        if (self.step_y > 0 && curr.1 > self.end.1) || (self.step_y < 0 && curr.1 < self.end.1) {
            return None;
        }
        Some(curr)
    }
}

fn load_data(contents: &str) -> Vec<CoordPair> {
    contents
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" -> ").unwrap();
            let from = left.split_once(',').unwrap();
            let to = right.split_once(',').unwrap();
            CoordPair {
                from: (from.0.parse().unwrap(), from.1.parse().unwrap()),
                to: (to.0.parse().unwrap(), to.1.parse().unwrap()),
            }
        })
        .collect()
}

fn solve_part_one(contents: &str) -> i32 {
    let mut intersections = HashMap::new();
    let data = load_data(contents);

    for line in data {
        if line.from.0 == line.to.0 || line.from.1 == line.to.1 {
            for point in line.iter() {
                *intersections.entry(point).or_insert(0) += 1;
            }
        }
    }

    let total_intersections = intersections
        .values()
        .fold(0, |acc, val| acc + (*val > 1) as i32);

    total_intersections
}

fn solve_part_two(contents: &str) -> i32 {
    let mut insersect = HashMap::new();
    let data = load_data(contents);

    for line in data {
        for point in line.iter() {
            *insersect.entry(point).or_insert(0) += 1;
        }
    }

    let total = insersect
        .values()
        .fold(0, |acc, val| acc + (*val > 1) as i32);

    total
}
