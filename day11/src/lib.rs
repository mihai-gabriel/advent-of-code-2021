//! Advent of Code 2021, Day 11
//! https://adventofcode.com/2021/day/11
//!
//! Șucaliuc Mihai-Gabriel
//! https://github.com/mihai-gabriel/
//!


use std::error::Error;
use std::fs;

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let matrix: Vec<Vec<u32>> = contents
        .lines()
        .map(|x| x.bytes().fold(vec![], |mut acc, val| {
            acc.push((val - 0x30) as u32);
            acc
        }))
        .collect();

    let part_one = solve_part_one(&matrix);
    println!("part one: {}", part_one);

    let part_two = solve_part_two(&matrix);
    println!("part two: {}", part_two);

    Ok(())
}

fn chain_reaction(m: &mut Vec<Vec<u32>>, v: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    if v[i][j] {
        return;
    }

    match m[i][j] {
        10 => {
            m[i][j] = 0;
            v[i][j] = true;
            // x - -
            // - c -    -> x = target; c = current
            // - - -
            if i > 0 && j > 0 { chain_reaction(m, v, i - 1, j - 1); }

            // - x -
            // - c -
            // - - -
            if i > 0 { chain_reaction(m, v, i - 1, j); }

            // - - x
            // - c -
            // - - -
            if i > 0 && j < m[i].len() - 1 { chain_reaction(m, v, i - 1, j + 1); }

            // - - -
            // - c x
            // - - -
            if j < m[i].len() - 1 { chain_reaction(m, v, i, j + 1); }

            // - - -
            // - c -
            // - - x
            if i < m.len() - 1 && j < m[i].len() - 1 { chain_reaction(m, v, i + 1, j + 1); }

            // - - -
            // - c -
            // - x -
            if i < m.len() - 1 { chain_reaction(m, v, i + 1, j); }

            // - - -
            // - c -
            // x - -
            if i < m.len() - 1 && j > 0 { chain_reaction(m, v, i + 1, j - 1); }

            // - - -
            // x c -
            // - - -
            if j > 0 { chain_reaction(m, v, i, j - 1); }
        }
        _ => {
            m[i][j] += 1;
            if m[i][j] == 10 {
                chain_reaction(m, v, i, j);
            }
        }
    }
}

fn solve_part_one(matrix: &[Vec<u32>]) -> u64 {
    let mut matrix = matrix.to_owned();
    let mut total = 0;
    for _ in 0..100 {
        let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if visited[i][j] { continue; }

                matrix[i][j] += 1;
                if matrix[i][j] == 10 {
                    chain_reaction(&mut matrix, &mut visited, i, j);
                }
            }
        }
        total += visited.iter().flatten().filter(|&x| *x).count() as u64;
    }

    total
}

fn solve_part_two(matrix: &[Vec<u32>]) -> u64 {
    let mut matrix = matrix.to_owned();
    let mut count = 0;
    loop {
        count += 1;
        let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if visited[i][j] { continue; }

                matrix[i][j] += 1;
                if matrix[i][j] == 10 {
                    chain_reaction(&mut matrix, &mut visited, i, j);
                }
            }
        }
        if visited.iter().flatten().filter(|&x| *x).count() == 100 {
            break;
        }
    }

    count
}

