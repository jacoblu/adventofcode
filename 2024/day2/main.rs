use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let mut answer = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Should be number"))
                .collect();

            answer += if is_report_safe(values) { 1 } else { 0 };
        }
    }

    println!("Part 1 answer is {}", answer)
}

fn is_report_safe(values: Vec<i32>) -> bool {
    let (min, max) = if values[0] < values[1] {
        (1, 3)
    } else {
        (-3, -1)
    };
    for i in 1..values.len() {
        let diff = values[i] - values[i - 1];
        if diff < min || diff > max {
            return false;
        }
    }
    true
}

fn part_two() {
    let mut answer = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let values: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Should be number"))
                .collect();

            if is_report_safe(values.clone()) {
                answer += 1;
            } else {
                for i in 0..values.len() {
                    let filtered: Vec<i32> = values
                        .iter()
                        .cloned()
                        .enumerate()
                        .filter(|&(ix, _)| ix != i)
                        .map(|(_, x)| x)
                        .collect();

                      if is_report_safe(filtered){
                        answer += 1;
                        break;
                      }
                }
            }
        }
    }
    println!("Part 2 answer is {}", answer)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
