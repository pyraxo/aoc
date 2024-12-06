use std::{
    collections::{HashMap, HashSet},
    io,
};

use aoc2024::util::read_char_input;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

const IS_TEST: bool = false;

fn main() -> io::Result<()> {
    // 0: empty, 1: wall, 2: visited
    let max_val = if IS_TEST { 10 } else { 130 };
    let mut board: Vec<Vec<u8>> = vec![vec![0; max_val]; max_val];

    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut dir: usize = 0;

    let mut visited_count = 0;
    let mut start_pos = (0, 0);

    let lines = read_char_input("6", IS_TEST)?;
    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            if chars[j] == '#' {
                board[i][j] = 1;
            } else if chars[j] == '^' {
                board[i][j] = 2;
                visited_count += 1;
                curr_x = i;
                curr_y = j;
                start_pos = (i, j);
            }
        }
    }

    let mut test_board = board.clone();
    let mut possible_obstacles: Vec<(usize, usize)> = Vec::new();
    while curr_x != 0 && curr_y != 0 && curr_x != max_val - 1 && curr_y != max_val - 1 {
        let next_x = (curr_x as i32 + DIRECTIONS[dir].0) as usize;
        let next_y = (curr_y as i32 + DIRECTIONS[dir].1) as usize;
        // println!("{} {}", next_x, next_y);
        if board[next_x][next_y] == 1 {
            // collect points we bump into obstacles
            dir = (dir + 1) % 4;
        } else {
            possible_obstacles.push((next_x, next_y));
            if board[curr_x][curr_y] == 0 {
                board[curr_x][curr_y] = 2;
                visited_count += 1;
            }
            curr_x = next_x;
            curr_y = next_y;
        }
    }
    visited_count += 1;
    println!("{}", visited_count);

    // brute force was too slow so we follow the original path
    let mut obstacles: HashSet<(usize, usize)> = HashSet::new();
    for (i, j) in possible_obstacles {
        curr_x = start_pos.0;
        curr_y = start_pos.1;
        if test_board[i][j] == 0 && (i, j) != start_pos {
            test_board[i][j] = 1;
            dir = 0;
            let mut steps = 0;
            let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::new();
            loop {
                if visited.contains_key(&(curr_x, curr_y, dir))
                    && steps - visited[&(curr_x, curr_y, dir)] > 0
                {
                    obstacles.insert((i, j));
                    break;
                }
                if curr_x <= 0 || curr_y <= 0 || curr_x >= max_val - 1 || curr_y >= max_val - 1 {
                    break;
                }
                let next_x = (curr_x as i32 + DIRECTIONS[dir].0) as usize;
                let next_y = (curr_y as i32 + DIRECTIONS[dir].1) as usize;
                visited.insert((curr_x, curr_y, dir), steps);
                if test_board[next_x][next_y] == 1 {
                    dir = (dir + 1) % 4;
                } else {
                    curr_x = next_x;
                    curr_y = next_y;
                    steps += 1;
                }
            }
            test_board[i][j] = 0;
            // println!("{} {}", i, j);
        }
    }
    println!("{:?}", obstacles.len());
    Ok(())
}
