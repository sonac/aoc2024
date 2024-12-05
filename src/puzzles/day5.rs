use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Input {
    rules: HashMap<i32, HashSet<i32>>,
    pages: Vec<Vec<i32>>
}
pub fn solve() {
    let input = parse_input();
    let result_p1 = sum_correct_middies(&input);
    println!("{}", result_p1);
    let result_p2 = sum_wrong_middies(&input);
    println!("{}", result_p2)
}

fn parse_input() -> Input {
    let input = fs::read_to_string(&"src/input/day5.txt".to_string()).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut pause_idx = 0;
    for (idx, line) in lines.iter().enumerate() {
        if *line == "" {
            pause_idx = idx+1;
            break
        }
        let nums = line.split_once('|').unwrap();
        rules.entry(nums.1.parse().unwrap())
            .or_insert(HashSet::from([nums.0.parse().unwrap()]))
            .insert(nums.0.parse().unwrap());
    }
    let mut pages: Vec<Vec<i32>> = Vec::new();
    for i in pause_idx..lines.len() {
        let parsed_line: Vec<i32> = lines.get(i).unwrap().split(',').map(|e| e.parse().unwrap()).collect();
        pages.push(parsed_line);
    }

    let parsed_input: Input = Input {rules, pages};
    parsed_input
}

fn sum_correct_middies(input: &Input) -> i32 {
    let mut result = 0;
    for page in &input.pages {
        if is_page_correct(page, &input.rules) {
            let mid = page[page.len()/2];
            result += mid;
        }
    }
    result
}

fn sum_wrong_middies(input: &Input) -> i32 {
    let mut result = 0;
    for page in &input.pages {
        if !is_page_correct(page, &input.rules) {
            let sorted_page = sort_page(page, &input.rules);
            let mid = sorted_page[sorted_page.len()/2];
            result += mid;
        }
    }
    result
}

fn is_page_correct(page: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut prev: HashSet<i32> = HashSet::from([page[0]]);
    for idx in 1..page.len() {
        let v = page[idx];
        if !rules.contains_key(&v) || !prev.is_subset(&rules[&v]) {
            return false
        }
        prev.insert(page[idx]);
    }
    true
}

fn sort_page(page: &Vec<i32>, rules: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut sorted_page: Vec<i32> = Vec::from([page[0]]);
    for idx in 1..page.len() {
        let v = page[idx];
        if !rules.contains_key(&v) {
            sorted_page.insert(0, v);
            continue
        }
        let cur_rules = rules.get(&v).unwrap();
        let mut inserted = false;
        for s_idx in 0..sorted_page.len() {
            if !cur_rules.contains(&sorted_page[s_idx]) {
                sorted_page.insert(s_idx, v);
                inserted = true;
                break
            }
        }
        if !inserted {
            sorted_page.push(v);
        }
    }
    sorted_page
}