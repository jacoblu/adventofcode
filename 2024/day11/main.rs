use std::{collections::HashMap, fs};

fn main() {
    let stones = read_input();

    part_one(&stones);

    part_two(&stones);
}

fn read_input() -> Vec<i64> {
    let input = fs::read_to_string("input").expect("Should read file");
    input.split(' ').map(|x| x.parse().unwrap()).collect()
}

fn part_one(stones: &Vec<i64>) {
    let mut new_stones = stones.clone();
    for _ in 0..25 {
        new_stones = blink(&new_stones);
    }

    println!("Part 1 answer is {}", new_stones.len())
}

fn blink(stones: &Vec<i64>) -> Vec<i64> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
            continue;
        }

        let stone_string = format!("{}", stone);
        if stone_string.len() % 2 == 0 {
            let split = split_stone(stone_string);
            new_stones.push(split[0]);
            new_stones.push(split[1]);
            continue;
        }

        new_stones.push(stone * 2024);
    }

    new_stones
}

fn split_stone(stone: String) -> [i64; 2] {
    let first = stone[..stone.len() / 2].parse().unwrap();
    let second = stone[stone.len() / 2..].parse().unwrap();

    [first, second]
}

fn part_two(stones: &Vec<i64>) {
    let mut new_stones: HashMap<i64, i64> = HashMap::new();
    for s in stones {
        new_stones.entry(*s).and_modify(|c| *c += 1).or_insert(1);
    }

    for _ in 0..75 {
        new_stones = blink_two(&new_stones);
    }
    let mut answer = 0;
    for (_, count) in new_stones {
        answer += count;
    }

    println!("Part 2 answer is {}", answer)
}

fn blink_two(stones: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut new_stones: HashMap<i64, i64> = HashMap::new();

    for (stone, count) in stones {
        if *stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|c| *c += count)
                .or_insert(*count);
            continue;
        }

        let stone_string = format!("{}", stone);
        if stone_string.len() % 2 == 0 {
            let split = split_stone(stone_string);
            new_stones
                .entry(split[0])
                .and_modify(|c| *c += count)
                .or_insert(*count);
            new_stones
                .entry(split[1])
                .and_modify(|c| *c += count)
                .or_insert(*count);
            continue;
        }

        new_stones
            .entry(stone * 2024)
            .and_modify(|c| *c += count)
            .or_insert(*count);
    }

    new_stones
}
