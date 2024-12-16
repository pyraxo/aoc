use std::{
    collections::{HashMap, HashSet},
    io,
};

use aoc2024::util::read_char_input;

const IS_TEST: bool = false;
const DIRECTIONS: [(i32, i32); 2] = [(-1, -1), (2, 2)];
const BOARD_SIZE: usize = if IS_TEST { 12 } else { 50 };

fn is_within_board(x: i32, y: i32) -> bool {
    x >= 0 && x < BOARD_SIZE as i32 && y >= 0 && y < BOARD_SIZE as i32
}

fn solve1(
    board: &Vec<Vec<char>>,
    freq_map: &HashMap<char, Vec<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (key, val) in freq_map {
        if val.len() < 2 {
            continue;
        }
        for i in 0..val.len() {
            for j in i + 1..val.len() {
                let (x1, y1) = val[i];
                let (x2, y2) = val[j];
                let offset_x: i32 = x2 as i32 - x1 as i32;
                let offset_y: i32 = y2 as i32 - y1 as i32;
                for dir in DIRECTIONS {
                    let new_x: i32 = x1 as i32 + offset_x * dir.0;
                    let new_y: i32 = y1 as i32 + offset_y * dir.1;
                    if is_within_board(new_x, new_y) {
                        if board[new_x as usize][new_y as usize] != *key {
                            antinodes.insert((new_x as usize, new_y as usize));
                        }
                    }
                }
            }
        }
    }
    antinodes
}

fn solve2(
    board: &Vec<Vec<char>>,
    freq_map: &HashMap<char, Vec<(usize, usize)>>,
) -> HashSet<(usize, usize)> {
    let mut t_nodes: HashSet<(usize, usize)> = HashSet::new();

    for (key, val) in freq_map {
        if val.len() < 2 {
            continue;
        }
        for i in 0..val.len() {
            for j in i + 1..val.len() {
                t_nodes.insert(val[i]);
                t_nodes.insert(val[j]);
                let (x1, y1) = val[i];
                let (x2, y2) = val[j];
                let offset_x: i32 = x2 as i32 - x1 as i32;
                let offset_y: i32 = y2 as i32 - y1 as i32;

                let mut curr_x = x1 as i32 - offset_x;
                let mut curr_y = y1 as i32 - offset_y;
                while is_within_board(curr_x, curr_y) {
                    if board[curr_x as usize][curr_y as usize] != *key {
                        t_nodes.insert((curr_x as usize, curr_y as usize));
                    }
                    curr_x -= offset_x;
                    curr_y -= offset_y;
                }

                curr_x = x1 as i32 + 2 * offset_x;
                curr_y = y1 as i32 + 2 * offset_y;

                while is_within_board(curr_x, curr_y) {
                    if board[curr_x as usize][curr_y as usize] != *key {
                        t_nodes.insert((curr_x as usize, curr_y as usize));
                    }
                    curr_x += offset_x;
                    curr_y += offset_y;
                }
            }
        }
    }
    t_nodes
}

fn main() -> io::Result<()> {
    let lines = read_char_input("8", IS_TEST)?;
    let mut board: Vec<Vec<char>> = vec![vec!['.'; BOARD_SIZE]; BOARD_SIZE];
    let mut freq_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            board[i][j] = chars[j];
            if chars[j] == '.' {
                continue;
            }
            freq_map.entry(chars[j]).or_insert(vec![]).push((i, j));
        }
    }

    println!("{}", solve1(&board, &freq_map).len());

    println!("{}", solve2(&board, &freq_map).len());
    Ok(())
}
