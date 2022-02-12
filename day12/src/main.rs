use std::collections::HashMap;

pub fn main() {
    let branches = include_str!("../data/input.txt")
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(HashMap::new(), |mut m, (a, b)| {
            m.entry(a).or_insert(Vec::with_capacity(6)).push(b);
            m.entry(b).or_insert(Vec::with_capacity(6)).push(a);
            m
        });

    println!("{:?}", branches);

    let mut vis = Vec::from(["end"]);
    let mut vis2 = vis.clone();
    vis.reserve(11);
    println!("{}", path(&branches, "end", &mut vis));
    println!("{}", path2(&branches, "end", &mut vis2, true));
}

fn path(m: &HashMap<&str, Vec<&'static str>>, cur: &str, vis: &mut Vec<&str>) -> usize {
    m.get(cur).unwrap().iter().fold(0, |acc, &b| match b {
        "start" => acc + 1,
        next if next.as_bytes()[0] > b'Z' && vis.contains(&next) => acc,
        next => {
            let len = vis.len();
            vis.push(next);
            let paths = path(m, next, vis);
            vis.truncate(len);
            acc + paths
        }
    })
}

fn path2(m: &HashMap<&str, Vec<&'static str>>, cur: &str, vis: &mut Vec<&str>, mul: bool) -> usize {
    m.get(cur).unwrap().iter().fold(0, |acc, &b| match b {
        "start" => acc + 1,
        "end" => acc,
        next => {
            let (uc, seen) = (next.as_bytes()[0] <= b'Z', vis.contains(&next));
            if !uc && !mul && seen {
                return acc;
            }

            let len = vis.len();
            vis.push(next);
            let paths = path2(m, next, vis, mul && (uc || !seen));
            vis.truncate(len);
            acc + paths
        }
    })
}