use std::{
    collections::{HashMap, VecDeque},
    io,
};

use aoc2024::util::read_char_input;

struct Rule {
    curr: u32,
    next: u32,
}

fn is_correct_order(rules: &Vec<Rule>, page: &Vec<u32>) -> bool {
    for i in 0..page.len() - 1 {
        // if B|A for A,B means false
        if rules
            .iter()
            .any(|rule| rule.curr == page[i + 1] && rule.next == page[i])
        {
            return false;
        }
    }
    true
}

fn get_correct_order(rules: &Vec<Rule>, pages: &Vec<u32>) -> Vec<u32> {
    // sub adj list
    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_deg: HashMap<u32, u32> = HashMap::new();
    let mut correct_order: Vec<u32> = vec![];
    // kahn's algorithm
    for &page in pages {
        adj_list.insert(page, vec![]);
        in_deg.insert(page, 0);
    }
    for rule in rules {
        if pages.contains(&rule.curr) && pages.contains(&rule.next) {
            adj_list.get_mut(&rule.curr).unwrap().push(rule.next);
            *in_deg.entry(rule.next).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<u32> = pages
        .iter()
        .filter(|&page| *in_deg.get(page).unwrap() == 0)
        .copied()
        .collect();

    while let Some(node) = queue.pop_front() {
        correct_order.push(node);
        for next_node in adj_list.get(&node).unwrap() {
            // decrease degree of all next nodes
            *in_deg.get_mut(next_node).unwrap() -= 1;
            if *in_deg.get(next_node).unwrap() == 0 {
                queue.push_back(*next_node);
            }
        }
    }
    if correct_order.len() == pages.len() {
        correct_order
    } else {
        vec![]
    }
}

fn main() -> io::Result<()> {
    let lines = read_char_input("5", false)?;
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut pages: Vec<Vec<u32>> = vec![];
    let mut rules: Vec<Rule> = vec![];

    for line in lines {
        if line.contains("|") {
            let (before, after) = line.split_once("|").unwrap();
            rules.push(Rule {
                curr: before.parse::<u32>().unwrap(),
                next: after.parse::<u32>().unwrap(),
            });
        } else if line.contains(",") {
            pages.push(
                line.split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    for page in &pages {
        if is_correct_order(&rules, page) {
            sum1 += page[page.len() / 2];
        } else {
            let order = get_correct_order(&rules, page);
            if order.len() > 0 {
                sum2 += order[order.len() / 2];
            }
        }
    }

    println!("{}", sum1);
    println!("{}", sum2);
    Ok(())
}
