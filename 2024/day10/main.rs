use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let map = read_input();

    part_one(&map);

    part_two(&map);
}

fn read_input() -> HashMap<i16, Vec<(i16, i16)>> {
    let mut map = HashMap::new();

    let mut y = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let mut x = 0;
            for c in line.chars() {
                let n = c.to_digit(10).unwrap() as i16;
                map.entry(n).or_insert(Vec::new()).push((x, y));
                x += 1;
            }
            y += 1;
        }
    }

    map
}

fn part_one(map: &HashMap<i16, Vec<(i16, i16)>>) {
    let mut answer = 0;

    for (x, y) in map.get(&0).unwrap() {
        let mut currents = HashSet::new();
        currents.insert((*x, *y));
        for i in 1..10 {
            let candidates = map.get(&i).unwrap();
            currents = find_next(&currents, candidates);
        }
        answer += currents.len();
    }

    println!("Part 1 answer is {}", answer)
}

fn find_next(currents: &HashSet<(i16, i16)>, candidates: &Vec<(i16, i16)>) -> HashSet<(i16, i16)> {
    let mut next = HashSet::new();
    for (x1, y1) in currents {
        for (x2, y2) in candidates {
            if ((x2 - x1).abs() == 1 && (y2 - y1) == 0) || ((x2 - x1) == 0 && (y2 - y1).abs() == 1)
            {
                next.insert((*x2, *y2));
            }
        }
    }

    next
}

fn part_two(map: &HashMap<i16, Vec<(i16, i16)>>) {
    let mut answer = 0;

    for (x, y) in map.get(&0).unwrap() {
        let mut currents = Vec::new();
        currents.push((*x, *y));
        for i in 1..10 {
            let candidates = map.get(&i).unwrap();
            currents = find_next_two(&currents, candidates);
        }
        answer += currents.len();
    }

    println!("Part 2 answer is {}", answer)
}

fn find_next_two(currents: &Vec<(i16, i16)>, candidates: &Vec<(i16, i16)>) -> Vec<(i16, i16)> {
    let mut next = Vec::new();
    for (x1, y1) in currents {
        for (x2, y2) in candidates {
            if ((x2 - x1).abs() == 1 && (y2 - y1) == 0) || ((x2 - x1) == 0 && (y2 - y1).abs() == 1)
            {
                next.push((*x2, *y2));
            }
        }
    }

    next
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
