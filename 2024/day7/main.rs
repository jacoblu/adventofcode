use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let equations = read_input();

    let (part_one_answer, invalid_equations) = part_one(&equations);

    part_two(part_one_answer, &invalid_equations);
}

fn read_input() -> Vec<(i64, Vec<i64>)> {
    let mut equations: Vec<(i64, Vec<i64>)> = Vec::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let line: Vec<&str> = line.split(": ").collect();
            let test_value = line[0].parse().unwrap();

            let numbers = line[1].split(' ').map(|m| m.parse().unwrap()).collect();

            equations.push((test_value, numbers));
        }
    };

    equations
}

fn part_one(equations: &Vec<(i64, Vec<i64>)>) -> (i64, Vec<(i64, Vec<i64>)>) {
    let mut answer = 0;
    let mut invalid_equations = Vec::new();

    for (test_value, numbers) in equations {
        if is_valid_part_one(*test_value, numbers[0], &numbers[1..].to_vec()) {
            answer += test_value;
        } else {
            invalid_equations.push((*test_value, numbers.clone()));
        }
    }

    println!("Part 1 answer is {}", answer);

    (answer, invalid_equations)
}

fn is_valid_part_one(target: i64, sum: i64, numbers: &Vec<i64>) -> bool {
    match numbers[..] {
        [n] => sum + n == target || sum * n == target,
        _ => {
            is_valid_part_one(target, sum + numbers[0], &numbers[1..].to_vec())
                || is_valid_part_one(target, sum * numbers[0], &numbers[1..].to_vec())
        }
    }
}

fn part_two(part_one_answer: i64, equations: &Vec<(i64, Vec<i64>)>) {
    let mut answer = part_one_answer;

    for (test_value, numbers) in equations {
        if is_valid_part_two(*test_value, numbers[0], &numbers[1..].to_vec()) {
            answer += test_value;
        }
    }

    println!("Part 2 answer is {}", answer)
}

fn is_valid_part_two(target: i64, sum: i64, numbers: &Vec<i64>) -> bool {
    match numbers[..] {
        [n] => sum + n == target || sum * n == target || combine(sum, n) == target,
        _ => {
            is_valid_part_two(target, sum + numbers[0], &numbers[1..].to_vec())
                || is_valid_part_two(target, sum * numbers[0], &numbers[1..].to_vec())
                || is_valid_part_two(target, combine(sum, numbers[0]), &numbers[1..].to_vec())
        }
    }
}

fn combine(first: i64, second: i64) -> i64 {
    format!("{}{}", first, second).parse().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
