use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| {
            line.split_once("   ")
                .and_then(|(x, y)| Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?)))
        })
        .unzip()
}

pub fn part_one(input: &str) -> Option<i32> {
    let (mut x, mut y): (Vec<i32>, Vec<i32>) = parse_input(input);

    x.sort();
    y.sort();

    let mut diff = 0;

    for i in 0..x.len() {
        diff += (x[i] - y[i]).abs();
    }
    Some(diff as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (x, y): (Vec<i32>, Vec<i32>) = parse_input(input);

    let mut freq_map: HashMap<i32, i32> = HashMap::new();
    for num in y {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let sim_score: i32 = x
        .iter()
        .map(|x: &i32| x * *freq_map.get(x).unwrap_or(&0) as i32)
        .sum();

    Some(sim_score as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
