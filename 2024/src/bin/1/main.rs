use std::io;

use aoc2024::util::read_input;

fn solve1() -> io::Result<()> {
    let lines = read_input("1")?;
    let mut x: Vec<i32> = vec![];
    let mut y: Vec<i32> = vec![];

    for split in lines {
        x.push(split[0]);
        y.push(split[1]);
    }

    x.sort();
    y.sort();

    let mut diff = 0;
    for i in 0..x.len() {
        diff += (x[i] - y[i]).abs();
    }
    println!("{}", diff);

    Ok(())
}

fn solve2() -> io::Result<()> {
    let lines = read_input("1")?;
    let mut x: Vec<i32> = vec![];
    let mut y: Vec<i32> = vec![0; 100000];

    for split in lines {
        x.push(split[0]);
        y[split[1] as usize] += 1;
    }

    let mut sum = 0;
    for i in 0..x.len() {
        sum += x[i] * y[x[i] as usize];
    }
    println!("{}", sum);

    Ok(())
}

fn main() -> io::Result<()> {
    solve1()?;
    solve2()?;
    Ok(())
}
