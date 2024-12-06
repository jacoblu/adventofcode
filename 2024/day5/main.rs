use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (rules, incorrect) = part_one();

    part_two(rules, incorrect);
}

fn part_one() -> (HashSet<(i16, i16)>, HashSet<Vec<i16>>) {
    let mut answer = 0;
    let mut rules: HashSet<(i16, i16)> = HashSet::new();

    let mut incorrect: HashSet<Vec<i16>> = HashSet::new();

    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let rule: Vec<&str> = line.split('|').collect();

            if rule.len() == 2 {
                let rule: Vec<i16> = rule
                    .into_iter()
                    .map(|m| m.parse::<i16>().unwrap())
                    .collect();

                rules.insert((rule[0], rule[1]));
                continue;
            } else if line.len() == 0 {
                continue;
            }

            let update: Vec<i16> = line.split(',').map(|m| m.parse::<i16>().unwrap()).collect();

            if is_in_order(update.clone(), &rules) {
                answer += update[update.len() / 2];
            } else {
                incorrect.insert(update);
            }
        }
    }

    println!("Part 1 answer is {}", answer);

    (rules, incorrect)
}

fn is_in_order(mut update: Vec<i16>, rules: &HashSet<(i16, i16)>) -> bool {
    let y = update.remove(0);

    for x in update.clone().into_iter() {
        if rules.contains(&(x, y)) {
            return false;
        }
    }

    match update[..] {
        [_] => true,
        _ => is_in_order(update, rules),
    }
}

fn part_two(rules: HashSet<(i16, i16)>, incorrect: HashSet<Vec<i16>>) {
    let mut answer = 0;

    for update in incorrect {
        let fixed = fix_update(rules.clone(), &update);

        answer += fixed[fixed.len() / 2];
    }

    println!("Part 2 answer is {}", answer)
}

fn fix_update(rules: HashSet<(i16, i16)>, update: &Vec<i16>) -> Vec<i16> {
    let mut first_ix = 0;

    if update.len() == 1 {
        return update.to_vec();
    }

    for mut i in 0..update.len() {
        let mut remaining = update.clone();
        remaining.remove(i);
        first_ix = i;

        let y = update[i];

        let mut can_be_first = true;

        for x in remaining {
            if rules.contains(&(x, y)) {
                can_be_first = false;
                break;
            }
        }

        if can_be_first {
            break;
        }
        i += 1;
    }

    let mut rest = update.clone();
    let first = rest.remove(first_ix);
    let mut correct = vec![first];

    let mut a = fix_update(rules, &rest);
    correct.append(&mut a);
    correct
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
