use std::fs;

pub fn solve() {
    let input = parse_input();
    let result_p1 = find_all_xmas(&input);
    println!("{}", result_p1);
    let result_p2 = find_all_x_mas(&input);
    println!("{}", result_p2);
}

fn parse_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(&"src/input/day4.txt".to_string()).unwrap();
    let lines: Vec<String> = input.lines().collect::<Vec<&str>>().iter().map(|l| l.to_string()).collect();
    let mut result = Vec::new();
    for line in lines {
        result.push(line.chars().collect::<Vec<char>>());
    }
    result
}

fn find_all_xmas(mtrx: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for y in 0..mtrx.len() {
        for x in 0..mtrx[0].len() {
            if mtrx[y][x] == 'X' {
                res += is_real_xmax(mtrx, x as i32, y as i32, 'X', 0);
            }
        }
    }
    res
}

fn find_all_x_mas(mtrx: &Vec<Vec<char>>) -> i32 {
    let mut res = 0;
    for y in 1..mtrx.len() - 1 {
        for x in 1..mtrx[0].len() - 1 {
            if mtrx[y][x] == 'A' && is_x_real_mas(mtrx, x, y) {
                res += 1;
            }
        }
    }
    res
}

// 1 -> right, 2 -> left, 3 -> top, 4 -> down, 5 -> right top, 6 -> left top, 7 -> right bottom, 8 -> left bottom
fn is_real_xmax(mtrx: &Vec<Vec<char>>, x: i32, y: i32, cur_letter: char, dir: i32) -> i32 {
    if x < 0 || x >= mtrx[0].len() as i32 || y < 0 || y >= mtrx.len() as i32 {
        return 0
    }
    if cur_letter == 'X' {
        let right = is_real_xmax(mtrx, x + 1, y, 'M', 1);
        let left = is_real_xmax(mtrx, x - 1, y, 'M', 2);
        let top = is_real_xmax(mtrx, x, y - 1, 'M', 3);
        let bot = is_real_xmax(mtrx, x, y + 1, 'M', 4);
        let rt = is_real_xmax(mtrx, x + 1, y - 1, 'M', 5);
        let lt = is_real_xmax(mtrx, x - 1, y - 1, 'M', 6);
        let rb = is_real_xmax(mtrx, x + 1, y + 1, 'M', 7);
        let lb = is_real_xmax(mtrx, x - 1, y + 1, 'M', 8);
        return right + left + top + bot + rt + lt + rb + lb;
    }
    if mtrx[y as usize][x as usize] == cur_letter {
        if cur_letter == 'S' {
            return 1
        }
        if dir == 1 {
            return is_real_xmax(mtrx, x + 1, y, get_next_letter(cur_letter), dir)
        } else if dir == 2 {
            return is_real_xmax(mtrx, x - 1, y, get_next_letter(cur_letter), dir)
        } else if dir == 3 {
            return is_real_xmax(mtrx, x, y - 1, get_next_letter(cur_letter), dir)
        } else if dir == 4 {
            return is_real_xmax(mtrx, x, y + 1, get_next_letter(cur_letter), dir)
        } else if dir == 5 {
            return is_real_xmax(mtrx, x + 1, y - 1, get_next_letter(cur_letter), dir)
        } else if dir == 6 {
            return is_real_xmax(mtrx, x - 1, y - 1, get_next_letter(cur_letter), dir)
        } else if dir == 7 {
            return is_real_xmax(mtrx, x + 1, y + 1, get_next_letter(cur_letter), dir)
        } else if dir == 8 {
            return is_real_xmax(mtrx, x - 1, y + 1, get_next_letter(cur_letter), dir)
        }
    }
    0
}

fn is_x_real_mas(mtrx: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut lr = false;
    let mut rl = false;
    // left to right
    if (mtrx[y-1][x-1] == 'S' && mtrx[y+1][x+1] == 'M') || (mtrx[y-1][x-1] == 'M' && mtrx[y+1][x+1] == 'S') {
        lr = true;
    }
    // right to left
    if (mtrx[y-1][x+1] == 'S' && mtrx[y+1][x-1] == 'M') || (mtrx[y-1][x+1] == 'M' && mtrx[y+1][x-1] == 'S') {
        rl = true;
    }
    lr && rl
}

fn get_next_letter(letter: char) -> char {
    if letter == 'X' {
        'M'
    } else if letter == 'M' {
        'A'
    } else {
        'S'
    }
}