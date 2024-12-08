use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (antennas, size) = read_input();

    part_one(&antennas, size);

    part_two(&antennas, size);
}

fn read_input() -> (HashMap<char, Vec<(i16, i16)>>, i16) {
    let mut antennas: HashMap<char, Vec<(i16, i16)>> = HashMap::new();

    let mut row = 0;
    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            let mut col = 0;
            for c in line.chars() {
                if c != '.' {
                    antennas.entry(c).or_insert(Vec::new()).push((row, col));
                }

                col += 1;
            }
            row += 1;
        }
    };

    (antennas, row)
}

fn part_one(antennas: &HashMap<char, Vec<(i16, i16)>>, size: i16) {
    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();

    for (_, coords) in antennas {
        for i in 0..coords.len() - 1 {
            let (row, col) = coords[i];
            for (row2, col2) in &coords[i + 1..] {
                // Find difference in position of the antennas
                let (row_diff, col_diff) = (row - row2, col - col2);

                // Try to add node on first antenna side
                let antinode = (row + row_diff, col + col_diff);
                if is_inbounds(antinode.0, antinode.1, size) {
                    antinodes.insert(antinode);
                }
                // Try to add node on second antenna side
                let antinode = (row2 - row_diff, col2 - col_diff);
                if is_inbounds(antinode.0, antinode.1, size) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    println!("Part 1 answer is {}", antinodes.len())
}

fn is_inbounds(row: i16, col: i16, size: i16) -> bool {
    row >= 0 && row < size && col >= 0 && col < size
}

fn part_two(antennas: &HashMap<char, Vec<(i16, i16)>>, size: i16) {
    let mut antinodes: HashSet<(i16, i16)> = HashSet::new();

    for (_, coords) in antennas {
        for i in 0..coords.len() - 1 {
            let (row, col) = coords[i];
            for (row2, col2) in &coords[i + 1..] {
                // Find difference in position of the antennas
                let (row_diff, col_diff) = (row - row2, col - col2);

                // Travel from antenna 1 in the direction of the calculated difference
                add_nodes_until_leaving_map(&mut antinodes, row, col, row_diff, col_diff, size);

                // Travel from antenna 2 in the opposite direction of the calculated difference
                add_nodes_until_leaving_map(
                    &mut antinodes,
                    *row2,
                    *col2,
                    -row_diff,
                    -col_diff,
                    size,
                );
            }
        }
    }

    println!("Part 2 answer is {}", antinodes.len())
}

fn add_nodes_until_leaving_map(
    antinodes: &mut HashSet<(i16, i16)>,
    row: i16,
    col: i16,
    row_diff: i16,
    col_diff: i16,
    size: i16,
) {
    let mut inbounds = true;
    let (mut new_row, mut new_col) = (row, col);
    while inbounds {
        if is_inbounds(new_row, new_col, size) {
            antinodes.insert((new_row, new_col));
            (new_row, new_col) = (new_row + row_diff, new_col + col_diff);
        } else {
            inbounds = false;
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
