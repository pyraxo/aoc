use std::io;

use aoc2024::util::read_input;

fn is_valid(split: &Vec<i32>) -> bool {
    let mut sorted = split.clone();
    sorted.sort();
    let mut reversed = sorted.clone();
    reversed.reverse();
    let mut flag = false;
    if split != &sorted && split != &reversed {
        return false;
    }
    for i in 0..split.len() - 1 {
        let diff = if split == &sorted {
            split[i + 1] - split[i]
        } else {
            split[i] - split[i + 1]
        };
        if diff < 1 || diff > 3 {
            flag = true;
            break;
        }
    }
    !flag
}

fn solve1() -> io::Result<()> {
    let lines = read_input("2", true)?;
    let mut counter = 0;

    for split in lines {
        if is_valid(&split) {
            counter += 1;
        }
    }
    println!("{}", counter);
    Ok(())
}

fn solve2() -> io::Result<()> {
    let lines = read_input("2", true)?;
    let mut counter = 0;

    for split in lines {
        if is_valid(&split) {
            counter += 1;
            continue;
        }
        for i in 0..split.len() {
            let mut new_split = split.clone();
            new_split.remove(i);
            if is_valid(&new_split) {
                counter += 1;
                break;
            }
        }
    }
    println!("{}", counter);
    Ok(())
}

fn main() -> io::Result<()> {
    solve1()?;
    solve2()?;
    Ok(())
}
