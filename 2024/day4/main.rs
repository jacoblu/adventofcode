use std::collections::HashSet;
use std::fs;

fn main() {
    part_one();

    part_two();
}

fn part_one() {
    let mut answer = 0;
    let contents: String = fs::read_to_string("input")
        .expect("Should read file")
        .split_whitespace()
        .collect();

    let length = (contents.len() as f64).sqrt() as i16;

    let mut x_s: HashSet<(i16, i16)> = HashSet::new();
    let mut m_s: HashSet<(i16, i16)> = HashSet::new();
    let mut a_s: HashSet<(i16, i16)> = HashSet::new();
    let mut s_s: HashSet<(i16, i16)> = HashSet::new();

    let mut row = 0;
    let contents: Vec<char> = contents.chars().collect();
    while row < length {
        let mut col = 0;
        while col < length {
            let ix = (row * length + col) as usize;

            match contents[ix] {
                'X' => x_s.insert((row, col)),
                'M' => m_s.insert((row, col)),
                'A' => a_s.insert((row, col)),
                'S' => s_s.insert((row, col)),
                _ => todo!(),
            };

            col += 1;
        }
        row += 1;
    }

    for (xrow, xcol) in x_s.clone().into_iter() {
        for (mrow, mcol) in m_s.clone().into_iter() {
            let row_diff = mrow - xrow;
            let col_diff = mcol - xcol;

            if (row_diff.abs() == 1 && col_diff.abs() == 1) || row_diff.abs() + col_diff.abs() == 1
            {
                for (arow, acol) in a_s.clone().into_iter() {
                    if arow - mrow == row_diff && acol - mcol == col_diff {
                        for (srow, scol) in s_s.clone().into_iter() {
                            if srow - arow == row_diff && scol - acol == col_diff {
                                answer += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Part 1 answer is {}", answer)
}

fn part_two() {
    let mut answer = 0;
    let contents: String = fs::read_to_string("input")
        .expect("Should read file")
        .split_whitespace()
        .collect();

    let length = (contents.len() as f64).sqrt() as i16;

    let mut a_s: HashSet<(i16, i16)> = HashSet::new();

    let mut row = 0;
    let contents: Vec<char> = contents.chars().collect();
    while row < length {
        let mut col = 0;
        while col < length {
            let ix = (row * length + col) as usize;

            match contents[ix] {
                'A' => a_s.insert((row, col)),
                _ => false,
            };

            col += 1;
        }
        row += 1;
    }

    for (arow, acol) in a_s.clone().into_iter() {
        //↖️M -> ↙️M or ↗️M or ↘️M
        let up_left = get_char(arow - 1, acol - 1, length, contents.clone());
        let down_left = get_char(arow + 1, acol - 1, length, contents.clone());
        let up_right = get_char(arow - 1, acol + 1, length, contents.clone());
        let down_right = get_char(arow + 1, acol + 1, length, contents.clone());

        let is_diagonal = match (up_left, up_right, down_left, down_right) {
            ('M', 'M', 'S', 'S') => true,
            ('M', 'S', 'M', 'S') => true,

            ('S', 'M', 'S', 'M') => true,
            ('S', 'S', 'M', 'M') => true,

            (_, _, _, _) => false,
        };

        if is_diagonal {
            answer += 1;
        }
    }

    println!("Part 2 answer is {}", answer)
}

fn get_char(row: i16, col: i16, length: i16, contents: Vec<char>) -> char {
    if row < 0 || row >= length || col < 0 || col >= length {
        return '.';
    }

    let ix = ((row * length) + col) as usize;

    contents[ix]
}
