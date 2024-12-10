use std::io;

use aoc2024::util::read_char_input;

const IS_TEST: bool = false;

const MAX_VAL: usize = if IS_TEST { 8 } else { 45 };

fn dfs(
    board: &Vec<Vec<u32>>,
    end_pts: &mut Vec<(usize, usize)>,
    start: (usize, usize),
    val: u32,
    find_unique: bool,
) -> u32 {
    let mut working_sum: u32 = 0;
    if board[start.0][start.1] == val {
        if board[start.0][start.1] == 9 {
            // println!("{}", board[start.0][start.1]);
            if !find_unique && end_pts.contains(&(start.0, start.1)) {
                // reject since we've reached before
                return 0;
            }
            end_pts.push((start.0, start.1));
            return 1;
        }
        if start.0 > 0 {
            // go up
            working_sum += dfs(
                &board,
                end_pts,
                (start.0 - 1, start.1),
                val + 1,
                find_unique,
            );
        }
        if start.1 > 0 {
            // go left
            working_sum += dfs(
                &board,
                end_pts,
                (start.0, start.1 - 1),
                val + 1,
                find_unique,
            );
        }
        if start.0 < MAX_VAL - 1 {
            // go down
            working_sum += dfs(
                &board,
                end_pts,
                (start.0 + 1, start.1),
                val + 1,
                find_unique,
            );
        }
        if start.1 < MAX_VAL - 1 {
            // go right
            working_sum += dfs(
                &board,
                end_pts,
                (start.0, start.1 + 1),
                val + 1,
                find_unique,
            );
        }
    }

    working_sum
}

fn solve1(board: &Vec<Vec<u32>>, starting_pts: &Vec<(usize, usize)>) -> io::Result<()> {
    let mut sum: u32 = 0;
    for pt in starting_pts {
        sum += dfs(&board, &mut vec![], *pt, 0, false);
    }
    println!("{}", sum);
    Ok(())
}

fn solve2(board: &Vec<Vec<u32>>, starting_pts: &Vec<(usize, usize)>) -> io::Result<()> {
    let mut sum: u32 = 0;
    for pt in starting_pts {
        sum += dfs(&board, &mut vec![], *pt, 0, true);
    }
    println!("{}", sum);
    Ok(())
}

fn main() -> io::Result<()> {
    let lines = read_char_input("10", IS_TEST).unwrap();
    let mut board: Vec<Vec<u32>> = vec![vec![0; MAX_VAL]; MAX_VAL];
    let mut starting_pts: Vec<(usize, usize)> = vec![];
    for i in 0..lines.len() {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len() {
            let val = chars[j].to_digit(10).unwrap();
            if val == 0 {
                starting_pts.push((i as usize, j as usize));
            }
            board[i][j] = val;
        }
    }
    solve1(&board, &starting_pts)?;
    solve2(&board, &starting_pts)?;
    Ok(())
}
