use std::io;
use std::io::Write;

fn main() {
    let input = include_str!("../data/input.txt");

    let part_one = part_one(&input);
    println!("{}", part_one);

    // let part_two = part_two(&input);
    // println!("{}", part_two);
}

fn folding_y(f_y: i32, vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let height = f_y * 2;

    let mut vec2: Vec<(i32, i32)> = vec.iter().filter(|(_, y)| y <= &f_y).copied().collect();
    let filtered_vec: Vec<(i32, i32)> = vec.iter().filter(|(_, y)| y > &f_y).map(|(x, y)| (*x, height - y)).collect();

    for coord in filtered_vec {
        if vec2.contains(&coord) {
            continue;
        }

        vec2.push(coord);
    }
    vec2
}

fn folding_x(f_x: i32, vec: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let width = f_x * 2;

    let mut vec2: Vec<(i32, i32)> = vec.iter().filter(|(x, _)| x <= &f_x).copied().collect();
    let filtered_vec: Vec<(i32, i32)> = vec.iter().filter(|(x, _)| x > &f_x).map(|(x, y)| (width - x, *y)).collect();

    for coord in filtered_vec {
        if vec2.contains(&coord) {
            continue;
        }

        vec2.push(coord);
    }
    vec2
}

fn part_one(text: &str) -> i32 {
    let (coords_text, folding) = text.split_once("\r\n\r\n").unwrap();
    let mut coords: Vec<(i32, i32)> = vec![];

    for coord in coords_text.lines() {
        let (x, y) = coord.split_once(',').unwrap();
        coords.push((x.parse().unwrap(), y.parse().unwrap()));
    }

    for line in folding.lines() {
        let (text, value) = line.split_once('=').unwrap();
        let axis = text.chars().last().unwrap();
        let value: i32 = value.parse().unwrap();

        match axis {
            'y' => coords = folding_y(value, coords),
            'x' => coords = folding_x(value, coords),
            _ => unreachable!()
        }
        // print here to get the first solution
    }

    let mut d_v = vec![vec![0i32; 50]; 50];

    for coord in coords.iter() {
        let (x, y) = coord;
        d_v[*x as usize][*y as usize] = 1;
    }

    for v in d_v {
        for elem in v {
            if elem == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        io::stdout().flush().unwrap();
        println!("");
    }

    // rotate and flip the image to figure out the solution.

    coords.len() as i32
}