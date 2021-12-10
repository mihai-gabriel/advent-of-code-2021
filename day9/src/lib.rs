//! advent of code 2021, day 9
//! https://adventofcode.com/2021/day/9
//!
//! șucaliuc mihai-gabriel
//! https://github.com/mihai-gabriel/
//!

use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let mut matrix: Vec<Vec<u32>> = contents
        .lines()
        .map(|x| x.bytes().fold(vec![], |mut acc, val| {
            acc.push((val - 0x30) as u32);
            acc
        }))
        .collect();

    let part_one = solve_part_one(&matrix);
    println!("part one: {}", part_one);

    let part_two = solve_part_two(&mut matrix);
    println!("part two: {}", part_two);

    Ok(())
}

fn solve_part_one(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut heightmap = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let nr = matrix[i][j];
            let neighbours = [
                if i > 0 { matrix[i - 1][j] } else { 10 },
                if j < matrix[i].len() - 1 { matrix[i][j + 1] } else { 10 },
                if i < matrix.len() - 1 { matrix[i + 1][j] } else { 10 },
                if j > 0 { matrix[i][j - 1] } else { 10 }
            ];

            if neighbours.iter().filter(|&x| *x <= nr).count() == 0 {
                heightmap.push(nr + 1);
            }
        }
    }

    heightmap.iter().sum::<u32>()
}

fn basin_size(matrix: &mut Vec<Vec<u32>>, i: usize, j: usize) -> u32 {
    match matrix[i][j] {
        9 | 10 => 0,
        _ => {
            matrix[i][j] = 10; // mark as visited so we won't go back to this value
            1 + if i > 0 { basin_size(matrix, i - 1, j) } else { 0 }
                + if j < matrix[i].len() - 1 { basin_size(matrix, i, j + 1) } else { 0 }
                + if i < matrix.len() - 1 { basin_size(matrix, i + 1, j) } else { 0 }
                + if j > 0 { basin_size(matrix, i, j - 1) } else { 0 }
        }
    }
}

fn solve_part_two(matrix: &mut Vec<Vec<u32>>) -> u32 {
    let mut result = vec![];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            match matrix[i][j] {
                9 | 10 => continue,
                _ => result.push(basin_size(matrix, i, j))
            }
        }
    }

    result.sort();
    result.iter().rev().take(3).product()
}