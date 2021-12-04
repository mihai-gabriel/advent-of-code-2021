//! Advent of Code 2021, Day 3
//! https://adventofcode.com/2021/day/3
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

fn solve_part_one(contents: &str) -> u32 {
    let gamma: u32 = contents
        .lines()
        .fold(vec![0u32; 12], |mut vec_acc, line| {
            line.chars().enumerate().for_each(|(i, digit)| vec_acc[i] += digit as u32 - 0x30);
            vec_acc
        })
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, &count)| {
            acc | if count >= 500 { 1 << (0xB - i) } else { 0 }
        });

    let epsilon = !gamma & 0xFFF;

    gamma * epsilon
}

fn more_bits(filtered: &Vec<Vec<u32>>, idx: usize) -> u32 {
    let count = filtered.iter().fold(0u32, |acc, line| acc + line[idx]);
    if count >= (filtered.len() - count as usize) as u32 { 1 } else { 0 }
}

fn fewer_bits(filtered: &Vec<Vec<u32>>, idx: usize) -> u32 {
    let count = filtered.iter().fold(0u32, |acc, line| acc + line[idx]);
    if count >= (filtered.len() - count as usize) as u32 { 0 } else { 1 }
}

fn solve_part_two(contents: &str) -> u32 {
    let contents: Vec<Vec<u32>> = contents
        .lines()
        .fold(vec![], |mut acc, line| {
            acc.push(line.as_bytes().iter().map(|x| *x as u32 - 0x30).collect());
            acc
        });

    let mut oxygen = contents.clone();
    for idx in 0..12 {
        if oxygen.len() == 1 { break; }
        let current_bit = more_bits(&oxygen, idx);
        oxygen.retain(|line| line[idx] == current_bit);
    }
    let oxygen = oxygen.last().unwrap().iter().fold(0u32, |acc, val| (acc << 1) + val);

    let mut co2 = contents.clone();
    for idx in 0..12 {
        if co2.len() == 1 { break; }
        let current_bit = fewer_bits(&co2, idx);
        co2.retain(|line| line[idx] == current_bit);
    }
    let co2 = co2.last().unwrap().iter().fold(0u32, |acc, val| (acc << 1) + val);

    oxygen * co2
}

// todo!(): tests / comments