use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (obstacles, guard, size) = read_input();

    part_one(&obstacles, guard, size);

    part_two(&obstacles, guard, size);
}

fn read_input() -> (HashSet<(i16, i16)>, (i16, i16, char), i16) {
    let mut obstacles: HashSet<(i16, i16)> = HashSet::new();
    let mut guard: (i16, i16, char) = (-1, -1, 'X');

    let mut height = 0;
    let mut width = 0;

    if let Ok(lines) = read_lines("input") {
        let mut row = 0;
        for line in lines.flatten() {
            let mut col = 0;
            for c in line.chars() {
                if c == '#' {
                    obstacles.insert((row, col));
                } else if c == '.' {
                } else {
                    guard = (row, col, c);
                }
                col += 1;
            }
            width = col;
            row += 1;
        }
        height = row;
    };

    if height != width {
        todo!("Map is not square");
    }

    (obstacles, guard, height)
}

fn part_one(obstacles: &HashSet<(i16, i16)>, guard: (i16, i16, char), size: i16) {
    let (mut row, mut col, mut dir) = guard;
    let mut guard_on_map = true;
    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert((row, col));

    while guard_on_map {
        (row, col, dir) = advance((row, col, dir), obstacles);

        if is_out_of_bounds((row, col), size) {
            guard_on_map = false;
        } else {
            visited.insert((row, col));
        }
    }

    println!("Part 1 answer is {}", visited.len())
}

fn is_out_of_bounds((row, col): (i16, i16), size: i16) -> bool {
    row < 0 || row >= size || col < 0 || col >= size
}

fn advance((row, col, dir): (i16, i16, char), obstacles: &HashSet<(i16, i16)>) -> (i16, i16, char) {
    let (new_row, new_col) = match dir {
        '^' => (row - 1, col),
        '>' => (row, col + 1),
        'v' => (row + 1, col),
        '<' => (row, col - 1),
        _ => todo!("Invalid direction"),
    };

    if obstacles.contains(&(new_row, new_col)) {
        advance((row, col, turn(dir)), obstacles)
    } else {
        (new_row, new_col, dir)
    }
}

fn turn(dir: char) -> char {
    match dir {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => todo!("Invalid direction"),
    }
}

fn part_two(obstacles: &HashSet<(i16, i16)>, guard: (i16, i16, char), size: i16) {
    let mut answer = 0;

    for row in 0..size {
        for col in 0..size {
            if obstacles.contains(&(row, col)) {
                continue;
            }

            let mut added_obstacles = obstacles.clone();
            added_obstacles.insert((row, col));

            let (mut row, mut col, mut dir) = guard;
            let mut guard_on_map = true;
            let mut visited: HashSet<(i16, i16, char)> = HashSet::new();
            visited.insert((row, col, dir));

            while guard_on_map {
                (row, col, dir) = advance((row, col, dir), &added_obstacles);

                if is_out_of_bounds((row, col), size) {
                    guard_on_map = false;
                } else {
                    if visited.contains(&(row, col, dir)) {
                        answer += 1;
                        break;
                    } else {
                        visited.insert((row, col, dir));
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
