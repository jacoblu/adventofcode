use regex::Regex;
use std::fs;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let contents = fs::read_to_string("input").expect("Should read file");

    let answer = multiply(contents);

    println!("Part 1 answer is {}", answer)
}

fn multiply(contents: String) -> i32 {

    let re = Regex::new(r"mul\([0-9]{1,3}\,[0-9]{1,3}\)").unwrap();

    let expressions: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut total = 0;
    for e in expressions {
        let e = &e[4..e.len()-1].split(',').map(|m| m.parse().expect("Should be number")).collect::<Vec<i32>>();

        total += e[0] * e[1];
    }

    total
}

fn part_two() {
    let contents = fs::read_to_string("input").expect("Should read file");

    let contents = "do()".to_owned() + &contents + "don't()";

    let re = Regex::new(r"do\(\)([\s\S]*?)don't\(\)").unwrap();

    let enabled_expressions: Vec<&str> = re.find_iter(&contents).map(|m| m.as_str()).collect();

    let mut answer = 0;
    for e in enabled_expressions {
        answer += multiply(e.to_string());
    }

    println!("Part 2 answer is {}", answer)
}
