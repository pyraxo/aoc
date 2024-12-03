use regex::Regex;
use std::io;

use aoc2024::util::read_full_input;

fn solve1() -> io::Result<()> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let text = &read_full_input("3")?;
    let mut sum = 0;
    let mut count = 0;

    for (_, [v1, v2]) in re.captures_iter(text).map(|c| c.extract()) {
        count += 1;
        sum += v1.parse::<u64>().unwrap() * v2.parse::<u64>().unwrap();
    }

    println!("number of matches: {}", count);
    println!("{}", sum);
    Ok(())
}

fn solve2() -> io::Result<()> {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
    let text = &read_full_input("3")?;
    let mut sum = 0;
    let mut count = 0;
    let mut do_count = true;

    for c in re.captures_iter(text) {
        count += 1;
        let (_, [val]) = c.extract();
        if val == "do()" {
            do_count = true;
        } else if val == "don't()" {
            do_count = false;
        } else if do_count {
            let str = &val[4..val.len() - 1];
            let vals = str.split(',').collect::<Vec<&str>>();
            sum += vals[0].parse::<u64>().unwrap() * vals[1].parse::<u64>().unwrap();
        }
    }

    println!("number of matches: {}", count);
    println!("{}", sum);
    Ok(())
}

fn main() -> io::Result<()> {
    solve1()?;
    solve2()?;
    Ok(())
}
