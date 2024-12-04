use std::fs;

pub fn solve() {
    let inp = parse_input();
    let p1_result = get_safe_lines(&inp);
    println!("{:?}", p1_result);
    let p2_result = get_all_safe_lines(&inp);
    println!("{:?}", p2_result);
}

fn parse_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string(&"src/input/day2.txt".to_string()).unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut res = Vec::<Vec<i32>>::new();
    for l in lines {
        res.push(l.split_whitespace().map(|s| s.parse().unwrap()).collect());
    }
    res
}

fn get_safe_lines(inp: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for l in inp {
        if is_line_safe(&l) {
            res += 1;
        }
    }
    res
}

fn get_all_safe_lines(inp: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for l in inp {
        if is_line_safe(&l) {
            res += 1;
        } else {
            for i in 0..l.len() {
                let new_line = [&l[0..i], &l[i+1..]].concat();
                if is_line_safe(&new_line) {
                    res += 1;
                    break
                }
            }
        }
    }
    res
}

fn is_line_safe(line: &Vec<i32>) -> bool {
    let mut res = true;
    let incr = get_trend(&line);
    for i in 0..line.len() - 1 {
        if line[i] > line[i + 1] && incr {
            res = false;
        }
        if line[i] < line[i + 1] && !incr {
            res = false;
        }
        if line[i] == line[i + 1] {
            res = false;
        }
        let diff = line[i] - line[i + 1];
        if diff.abs() > 3 {
           res = false;
        }
    }
    res
}

// true increasing, false - decreasing
fn get_trend(line: &Vec<i32>) -> bool {
    for i in 0..line.len() - 2 {
        if line[i] > line[i + 1] {
            return false;
        }
        if line[i] < line[i + 1] {
            return true
        }
    }
    true
}