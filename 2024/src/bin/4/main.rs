use std::io;

use aoc2024::util::read_char_input;

const KEYWORD: &str = "XMAS";

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

const M_POSITIONS: [[(i32, i32); 2]; 4] = [
    [(-1, -1), (1, -1)],
    [(-1, -1), (-1, 1)],
    [(-1, 1), (1, 1)],
    [(1, -1), (1, 1)],
];

const S_POSITIONS: [[(i32, i32); 2]; 4] = [
    [(-1, 1), (1, 1)],
    [(1, -1), (1, 1)],
    [(-1, -1), (1, -1)],
    [(-1, -1), (-1, 1)],
];

fn dfs(
    board: &Vec<Vec<String>>,
    char_index: usize,
    i: usize,
    j: usize,
    direction: Option<(i32, i32)>,
) -> i32 {
    if i >= board.len() || j >= board[i].len() {
        return 0;
    }
    if board[i][j] == KEYWORD.chars().nth(char_index).unwrap().to_string() {
        if char_index == KEYWORD.len() - 1 {
            return 1;
        }
        let mut total = 0;
        if let Some((di, dj)) = direction {
            let new_i = (i as i32 + di) as usize;
            let new_j = (j as i32 + dj) as usize;
            total += dfs(board, char_index + 1, new_i, new_j, Some((di, dj)));
        } else {
            for &(di, dj) in DIRECTIONS.iter() {
                total += dfs(
                    board,
                    char_index + 1,
                    (i as i32 + di) as usize,
                    (j as i32 + dj) as usize,
                    Some((di, dj)),
                );
            }
        }
        return total;
    }
    0
}

fn solve1(board: &Vec<Vec<String>>) -> io::Result<()> {
    let mut count = 0;
    for j in 0..board[0].len() {
        for i in 0..board.len() {
            if board[i][j] == "X" {
                count += dfs(&board, 0, i, j, None);
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn check_char(
    board: &Vec<Vec<String>>,
    i: usize,
    j: usize,
    char: &str,
    positions: &[(i32, i32); 2],
) -> bool {
    for &(di, dj) in positions.iter() {
        let new_i = (i as i32 + di) as usize;
        let new_j = (j as i32 + dj) as usize;
        if new_i >= board.len() || new_j >= board[new_i].len() {
            return false;
        }
        if board[new_i][new_j] != char {
            return false;
        }
    }
    true
}

fn solve2(board: &Vec<Vec<String>>) -> io::Result<()> {
    let mut count = 0;
    for j in 0..board[0].len() {
        for i in 0..board.len() {
            if board[i][j] == "A" {
                for k in 0..M_POSITIONS.len() {
                    if check_char(board, i, j, "M", &M_POSITIONS[k])
                        && check_char(board, i, j, "S", &S_POSITIONS[k])
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("{}", count);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut board = vec![vec![String::new(); 140]; 140];
    let lines = read_char_input("4", false)?;
    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            board[i][j] = chars[j].to_string();
        }
    }
    solve1(&board)?;
    solve2(&board)?;
    Ok(())
}
