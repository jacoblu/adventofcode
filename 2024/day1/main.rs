use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
  part_one();

  part_two();
}

fn part_one() {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("input") {
      for line in lines.flatten() {
        let values = line.split("   ").collect::<Vec<&str>>();

        left_list.push(values[0].parse().expect(""));
        right_list.push(values[1].parse().expect(""));
      }
    }

    left_list.sort();
    right_list.sort();

    let mut answer = 0;
    for i in 0..left_list.len()
    {
        let diff = (left_list[i] - right_list[i]).abs();
        answer += diff;
    }

    println!("Part 1 answer is {}", answer)
}

fn part_two() {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: HashMap<i32, i32> = HashMap::new();

    if let Ok(lines) = read_lines("input") {
      for line in lines.flatten() {
        let values = line.split("   ").collect::<Vec<&str>>();

        left_list.push(values[0].parse().expect(""));

        let right_value = values[1].parse().expect("");

        right_list.entry(right_value)
          .and_modify(|sum| *sum += 1)
          .or_insert(1);
      }
    }

    let mut answer = 0;
    for value in left_list
    {
        if right_list.contains_key(&value)
        {
            answer += value * right_list[&value];
        }
    }

    println!("Part 2 answer is {}", answer)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
