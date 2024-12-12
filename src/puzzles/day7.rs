use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = parse_input();
    let result_p1 = find_good_totals(input.clone());
    println!("{:?}", result_p1);
    let result_p2 = find_worse_totals(input.clone());
    println!("{:?}", result_p2)
}

fn parse_input() -> HashMap<i64, Vec<i32>> {
    let input = fs::read_to_string(&"src/input/day7.txt".to_string()).unwrap();
    let lines = input.lines();
    let mut result: HashMap<i64, Vec<i32>> = HashMap::new();
    for l in lines {
        let k = l.split(":").collect::<Vec<&str>>().get(0).unwrap().parse::<i64>().unwrap();
        let v: Vec<i32> = l.split(":").collect::<Vec<&str>>().get(1).unwrap()
            .split(" ").collect::<Vec<&str>>().iter()
            .filter(|el| !el.is_empty())
            .map(|el| el.parse::<i32>().unwrap()).collect();
        result.insert(k.clone(), v.clone());
    }
    result
}

fn find_good_totals(input: HashMap<i64, Vec<i32>>) -> i64 {
    let mut result = 0;
    for el in input {
        if brutator(el.0 as f64, el.1.clone()) {
            result += el.0;
        }
    }
    result
}

fn find_worse_totals(input: HashMap<i64, Vec<i32>>) -> i64 {
    let mut result = 0;
    for el in input {
        if brut(0, el.0, 0, &el.1) {
                result += el.0;
        }
    }
    result
}

fn brutator(target: f64, nums: Vec<i32>) -> bool {
    if target == 0.0 && nums.len() == 0 {
        return true
    }
    if nums.len() == 0 || target <= 0.0 {
        return false
    }
    let mut new_nums = nums.clone();
    let f = new_nums.pop().unwrap();
    brutator(target - f as f64, new_nums.clone()) || brutator(target / f as f64, new_nums.clone())
}

fn brut(idx: usize, target: i64, cur: i64, nums: &Vec<i32>) -> bool {
    if idx == nums.len() || cur > target {
        return cur == target
    }
    let concat: i64 = (cur.to_string() + nums[idx].to_string().as_str()).parse::<i64>().unwrap();
    brut(idx + 1, target, cur + nums[idx] as i64, nums) ||
        brut(idx + 1, target, cur * nums[idx] as i64, nums) ||
        brut(idx + 1, target, concat, nums)
}