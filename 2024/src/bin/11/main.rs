use std::{
    collections::HashMap,
    io::{self},
};

use aoc2024::util::read_full_input;

const IS_TEST: bool = false;

// const MAX_VAL: usize = if IS_TEST { 8 } else { 45 };

type Memo = HashMap<u64, u64>;

fn blink(memo: &mut Memo) -> &mut Memo {
    for (key, number) in memo.clone().iter() {
        let key_str = key.to_string();
        if key_str == "0" {
            memo.entry(1)
                .and_modify(|v| *v += number)
                .or_insert(*number);
        } else if key_str.len() % 2 == 0 {
            let first_num = key_str[..key_str.len() / 2].parse::<u64>().unwrap();
            let second_num = key_str[key_str.len() / 2..].parse::<u64>().unwrap();
            memo.entry(first_num)
                .and_modify(|v| *v += number)
                .or_insert(*number);
            memo.entry(second_num)
                .and_modify(|v| *v += number)
                .or_insert(*number);
        } else {
            let new_key = key * 2024;
            memo.entry(new_key)
                .and_modify(|v| *v += number)
                .or_insert(*number);
        }
        memo.entry(*key).and_modify(|v| *v -= number);
    }
    memo
}

fn main() -> io::Result<()> {
    let line = read_full_input("11", IS_TEST).unwrap();
    let mut memo_map: HashMap<u64, u64> = HashMap::new();
    let rocks: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    for rock in rocks {
        *memo_map.entry(rock).or_insert(0) += 1;
    }
    for _ in 0..25 {
        blink(&mut memo_map);
    }
    println!("{:?}", memo_map.values().sum::<u64>());
    for _ in 25..75 {
        blink(&mut memo_map);
    }
    println!("{:?}", memo_map.values().sum::<u64>());
    Ok(())
}
