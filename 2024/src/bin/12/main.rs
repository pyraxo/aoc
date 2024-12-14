use std::{
    collections::{HashSet, VecDeque},
    io,
};

use aoc2024::util::read_board;

const IS_TEST: bool = false;

type RegionCells = HashSet<(usize, usize)>;

const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() -> io::Result<()> {
    let board = read_board("12", IS_TEST).unwrap();
    let mut found: HashSet<(usize, usize)> = HashSet::new();
    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    let mut regions: Vec<RegionCells> = Vec::new();
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if found.contains(&(i, j)) {
                continue;
            }

            let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
            let mut current_area: u32 = 0;
            let mut current_perimeter: u32 = 0;
            let mut region_cells: RegionCells = HashSet::new();

            found.insert((i, j));
            stack.push_back((i, j));
            region_cells.insert((i, j));

            let current_type: String = board[i][j].clone();
            while let Some((i, j)) = stack.pop_front() {
                current_area += 1;
                for &(di, dj) in DIRS.iter() {
                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;
                    if new_i < 0
                        || new_j < 0
                        || new_i >= board.len() as i32
                        || new_j >= board[new_i as usize].len() as i32
                    {
                        current_perimeter += 1;
                        continue;
                    }

                    if board[new_i as usize][new_j as usize] != current_type {
                        current_perimeter += 1;
                        continue;
                    }
                    if !found.contains(&(new_i as usize, new_j as usize)) {
                        found.insert((new_i as usize, new_j as usize));
                        stack.push_back((new_i as usize, new_j as usize));
                        region_cells.insert((new_i as usize, new_j as usize));
                    }
                }
            }

            total1 += current_area * current_perimeter;
            regions.push(region_cells);
        }
    }
    println!("{}", total1);

    for region in regions {
        let mut corner_count: u32 = 0;
        for &(i, j) in region.iter() {
            let empty_count = DIRS
                .iter()
                .map(|(di, dj)| (i as i32 + di, j as i32 + dj))
                .filter(|(i, j)| !region.contains(&(*i as usize, *j as usize)))
                .count();
            corner_count += match empty_count {
                4 => 4, // isolated block
                3 => 2, // corner with 2 edges
                2 => {
                    // one block bridge
                    if (i > 0 && region.contains(&(i - 1, j)) && region.contains(&(i + 1, j)))
                        || (j > 0 && region.contains(&(i, j - 1)) && region.contains(&(i, j + 1)))
                    {
                        0
                    } else {
                        1
                    }
                }
                _ => 0,
            };
            // bottom right is a new internal region
            if !region.contains(&(i + 1, j + 1))
                && region.contains(&(i + 1, j))
                && region.contains(&(i, j + 1))
            {
                corner_count += 1;
            }
            // bottom left
            if j > 0
                && !region.contains(&(i + 1, j - 1))
                && region.contains(&(i + 1, j))
                && region.contains(&(i, j - 1))
            {
                corner_count += 1;
            }
            // top right
            if i > 0
                && !region.contains(&(i - 1, j + 1))
                && region.contains(&(i - 1, j))
                && region.contains(&(i, j + 1))
            {
                corner_count += 1;
            }
            // top left
            if i > 0
                && j > 0
                && !region.contains(&(i - 1, j - 1))
                && region.contains(&(i - 1, j))
                && region.contains(&(i, j - 1))
            {
                corner_count += 1;
            }
        }
        println!("{} {}", corner_count, region.len());
        total2 += corner_count * region.len() as u32;
    }
    println!("{}", total2);
    Ok(())
}
