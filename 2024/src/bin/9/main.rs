use std::{collections::HashMap, io};

use aoc2024::util::read_full_input;

const IS_TEST: bool = false;

type DiskEntryMap = HashMap<u32, DiskEntry>;

struct DiskEntry {
    size: u32,
    pos: usize,
}

fn part1_checksum(disk_orig: &Vec<i32>, used_spaces: u32) -> u64 {
    let mut checksum: u64 = 0;
    let mut disk = disk_orig.clone();
    let mut back_pointer: usize = disk.len() - 1;
    for i in 0..used_spaces {
        let curr_space = disk[i as usize];
        if curr_space != -1 {
            checksum += curr_space as u64 * i as u64;
            continue;
        }
        while disk[back_pointer] == -1 {
            back_pointer -= 1;
        }
        checksum += disk[back_pointer] as u64 * i as u64;
        disk[i as usize] = disk[back_pointer];
        disk[back_pointer] = -1;
    }
    checksum
}

fn part2_checksum(disk_orig: &Vec<i32>, disk_entries: &DiskEntryMap, total_ids: &u32) -> u64 {
    let mut checksum: u64 = 0;
    let mut back_id: u32 = *total_ids - 1;
    let mut disk = disk_orig.clone();
    // let mut blanks = blanks_orig.clone();
    while back_id > 0 {
        let entry = disk_entries.get(&back_id).unwrap();
        for i in 0..entry.pos as usize {
            if disk[i] != -1 {
                continue;
            }
            let slice = &disk[i..(i + entry.size as usize)];
            if slice.iter().all(|&x| x == -1) {
                for j in 0..slice.len() {
                    disk[i + j] = back_id as i32;
                    disk[entry.pos + j] = -1;
                }
                break;
            }
        }
        back_id -= 1;
    }
    for i in 0..disk.len() {
        let curr_space = disk[i];
        if curr_space != -1 {
            checksum += curr_space as u64 * i as u64;
        }
    }
    checksum
}

fn main() -> io::Result<()> {
    let input = read_full_input("9", IS_TEST)?;
    let mut disk: Vec<i32> = vec![];
    let mut curr_id: u32 = 0;
    let mut used_spaces: u32 = 0;
    let mut disk_entries: DiskEntryMap = HashMap::new(); // id -> {size, pos}
    for (i, char) in input.chars().enumerate() {
        if char == '\n' {
            continue;
        }
        if i % 2 == 0 {
            let used_size = char.to_digit(10).unwrap();
            disk_entries.insert(
                curr_id,
                DiskEntry {
                    size: used_size,
                    pos: disk.len(),
                },
            );
            for _ in 0..used_size {
                disk.push(curr_id as i32);
            }
            curr_id += 1;
            used_spaces += used_size;
        } else {
            let empty_size = char.to_digit(10).unwrap();
            for _ in 0..empty_size {
                disk.push(-1);
            }
        }
    }

    println!("{}", part1_checksum(&disk, used_spaces));
    println!("{}", part2_checksum(&disk, &disk_entries, &curr_id));

    Ok(())
}
