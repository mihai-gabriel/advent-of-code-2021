//! Advent of Code 2021, Day 4
//! https://adventofcode.com/2021/day/4
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

fn check_solved(matrix: &Vec<i32>) -> bool {
    for i in 0..5 {
        let line: i32 = matrix[0 + i * 5..5 + i * 5].iter().sum();
        let column: i32 = (0..5)
            .fold(vec![], |mut acc, j| {
                acc.push(matrix[i + j * 5]);
                acc
            }).iter().sum();

        // if line or column == [-1, -1, -1, -1, -1]
        if line == -5 || column == -5 {
            return true;
        }
    }
    false
}

fn load_data(contents: &str) -> Vec<Vec<i32>> {
    let mut matrixes: Vec<Vec<i32>> = vec![];
    let mut lines = contents.lines().skip(1); // skip to-draw numbers

    while let Some(_line) = lines.next() {
        let mut matrix: Vec<i32> = Vec::with_capacity(25);
        for _ in 0..5 {
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .for_each(|nr| matrix.push(nr));
        }
        matrixes.push(matrix);
    }
    matrixes
}

fn load_numbers(contents: &str) -> Vec<i32> {
    contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve_part_one(contents: &str) -> i32 {
    let all: Vec<i32> = load_numbers(&contents);
    let mut drawn: i32;
    let mut winner: i32 = -1;
    let mut sum = 0;

    // load data into vectors
    let mut matrixes = load_data(contents);

    'matrixes: for n in all {
        drawn = n;
        for matrix in matrixes.iter_mut() {
            // mark matrix with -1
            matrix.iter_mut().for_each(|nr| if *nr == n { *nr = -1 });
            sum = matrix.iter().filter(|&x| *x != -1).sum();

            // first occurrence should 'return' the winner
            if check_solved(&matrix) {
                winner = drawn;
                break 'matrixes;
            }
        }
    }

    winner * sum
}

fn solve_part_two(contents: &str) -> i32 {
    let all: Vec<i32> = load_numbers(&contents);
    let mut drawn: i32;
    let mut winner: i32 = -1;
    let mut sum = 0;

    // load data into vectors
    let mut matrixes = load_data(contents);

    for n in all {
        drawn = n;
        for matrix in matrixes.iter_mut() {
            // ignore matrix is already solved
            // we don't want to mark it again
            // (we do not break at first occurrence)
            if check_solved(&matrix) {
                continue;
            }

            // mark matrix
            matrix.iter_mut().for_each(|nr| if *nr == n { *nr = -1 });
            sum = matrix.iter().filter(|&x| *x != -1).sum();

            // check if matrix has been solved this iteration
            if check_solved(&matrix) {
                winner = drawn;
                // we do not break in order to override
                // until the last one that has been solved.
            }
        }
    }

    winner * sum
}
