use std::fs;
use regex::Regex;

pub fn solve() {
    let input = parse_input();
    let muls = fetch_muls(&input);
    let result_p1 = calc_muls(muls);
    println!("{:#?}", result_p1);
    let cond_muls = fetch_cond_muls(&input);
    let result_p2 = calc_muls(cond_muls);
    println!("{:#?}", result_p2);
}

fn parse_input() -> String {
    fs::read_to_string(&"src/input/day3.txt".to_string()).unwrap()
}

fn fetch_muls(inp: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let caps = re.captures_iter(inp);
    caps.map(|c| c.get(1).unwrap().as_str().to_owned()).collect::<Vec<String>>()
}

fn fetch_cond_muls(inp: &str) -> Vec<String> {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(do\(\))|(don\'t\(\))").unwrap();
    let caps = re.captures_iter(inp);
    let mut cond = true;
    let mut res = Vec::<String>::new();
    for capture in caps.into_iter() {
        for (idx, c) in capture.iter().enumerate().skip(1) {
            if let Some(mat) = c {
                if idx == 1 && cond {
                    res.push(mat.as_str().to_owned());
                }
                if idx == 2 {
                    cond = true;
                }
                if idx == 3 {
                    cond = false;
                }
            }
        }
    }
    res
}

fn calc_muls(muls: Vec<String>) -> i32 {
    let mut res = 0;
    for mul in muls {
        res += calc_mul(mul);
    }
    res
}

fn calc_mul(mul: String) -> i32 {
    let re = Regex::new(r"(\d{1,3})").unwrap();
    let caps = re.captures_iter(&mul);
    let parsed_caps = caps.map(|c| c.get(1).unwrap().as_str().parse().unwrap()).collect::<Vec<i32>>();
    parsed_caps[0] * parsed_caps[1]
}