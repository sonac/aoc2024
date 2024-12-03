use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let mut inp = parse_input();
    let p1_result = sort_and_calculate_min_dist(&mut inp.0, &mut inp.1);
    println!("{}", p1_result);
    let p2_result = calculate_similarity_score(&inp.0, &inp.1);
    println!("{}", p2_result);
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(&"src/input/day1.txt".to_string()).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for l in lines {
        let v = l.split_whitespace().collect::<Vec<&str>>();
        let f = v.first().unwrap();
        let l = v.last().unwrap();
        v1.push(f.parse::<i32>().unwrap());
        v2.push(l.parse::<i32>().unwrap());
    }
   (v1, v2)
}

fn sort_and_calculate_min_dist(v1: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    v1.sort();
    v2.sort();
    let mut result = 0;
    for i in 0..v1.len() {
        let diff = (v1.get(i).unwrap() - v2.get(i).unwrap()).abs() as i32;
        result += diff;
    }
    result
}

fn calculate_similarity_score(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let occ = build_hm(v2);
    let mut result = 0;
    for v in v1 {
        result += occ.get(v).unwrap_or(&0) * v;
    }
    result
}

fn build_hm(inp: &Vec<i32>) -> HashMap<i32, i32> {
    let mut hm = HashMap::new();
    for v in inp {
        hm.entry(*v).and_modify(|v| *v += 1).or_insert(1);
    }
    hm
}