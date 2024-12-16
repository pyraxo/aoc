use std::{collections::VecDeque, io};

use aoc2024::util::read_char_input;

const IS_TEST: bool = false;

fn solve(lines: &Vec<String>, with_concat: bool) -> u64 {
    let mut sum = 0;
    let mut working_nums: VecDeque<u64> = VecDeque::new();
    for line in lines {
        let vals: Vec<&str> = line.split_whitespace().collect();
        let goal: u64 = vals[0][0..vals[0].len() - 1].parse().unwrap();
        let mut nums: VecDeque<u64> = vals[1..].iter().map(|s| s.parse().unwrap()).collect();
        let first_num = nums.pop_front().unwrap();
        working_nums.push_back(first_num);
        while !nums.is_empty() {
            let next_num = nums.pop_front().unwrap();
            for _ in 0..working_nums.len() {
                let next_working_num = working_nums.pop_front().unwrap();
                working_nums.push_back(next_working_num + next_num);
                working_nums.push_back(next_working_num * next_num);
                if with_concat {
                    let concat_num = format!("{}{}", next_working_num, next_num);
                    working_nums.push_back(concat_num.parse().unwrap());
                }
            }
        }
        if working_nums.contains(&goal) {
            sum += goal;
        }
        working_nums.clear();
    }
    sum
}

fn main() -> io::Result<()> {
    let lines = read_char_input("7", IS_TEST)?;
    println!("{}", solve(&lines, false));
    println!("{}", solve(&lines, true));
    Ok(())
}
