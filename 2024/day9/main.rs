use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Should read file");

    part_one(&input);

    part_two(&input);
}

fn part_one(input: &str) {
    let mut blocks: Vec<i64> = Vec::new();

    let mut id: i64 = 0;
    let mut is_free = false;
    let mut nr_of_empty_blocks = 0;
    for c in input.chars() {
        let size: i64 = c.to_digit(10).unwrap().into();

        let mut fill = -1;
        if is_free {
            nr_of_empty_blocks += size;
        } else {
            fill = id;
            id += 1;
        }

        for _ in 0..size {
            blocks.push(fill);
        }

        is_free = !is_free;
    }

    for i in ((blocks.len() - nr_of_empty_blocks as usize)..blocks.len()).rev() {
        let c = blocks[i];
        if c < 0 {
            continue;
        }

        let first_empty = blocks.iter().position(|x| *x == -1).unwrap();
        blocks.swap(first_empty, i);
    }

    let mut answer = 0;
    for i in 0..(blocks.len() - nr_of_empty_blocks as usize) {
        answer += blocks[i] * i as i64;
    }

    println!("Part 1 answer is {}", answer)
}

fn part_two(input: &str) {
    let mut files: Vec<(i64, i64, i64)> = Vec::new(); //0: id, 1: size, 2: start
    let mut free: Vec<(i64, i64)> = Vec::new(); //0: size, 1: start

    let mut is_free = false;
    let mut id: i64 = 0;

    let mut index: i64 = 0;
    for c in input.chars() {
        let size: i64 = c.to_digit(10).unwrap().into();

        if is_free {
            if size > 0 {
                free.push((size, index));
            }
        } else {
            files.push((id, size, index));
            id += 1;
        }
        index += size;
        is_free = !is_free;
    }
    println!("files: {:?}", files);
    println!("free: {:?}", free);

    let mut moved_files: Vec<(i64, i64, i64)> = Vec::new();
    let mut reversed = files.clone();
    reversed.reverse();
    for (id, size, start) in reversed {
        // println!("{} {} {}", id, size, start);

        let mut new_start = start;
        for i in 0..free.len() {
            let (free_size, free_start) = free[i];
            if free_size >= size {
                new_start = free_start;

                if size == free_size {
                    free.remove(i);
                } else {
                    free[i] = (free_size - size, free_start + size);
                }
                break;
            }
        }

        moved_files.push((id, size, new_start));
    }
    println!("{:?}", moved_files);

    let mut answer = 0;
    for (id, size, start) in moved_files {
        for i in start..start + size {
            answer += id * i;
        }
    }
    println!("Part 2 answer is {}", answer)
}
