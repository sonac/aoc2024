use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input();
    let result_p1 = walk(&input);
    println!("{}", result_p1.0);
    let result_p2 = walk_in_loops(&input, &result_p1.1);
    println!("{}", result_p2);
}

fn parse_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(&"src/input/day6.txt".to_string()).unwrap();
    let lines : Vec<&str> = input.lines().collect();
    lines.iter().map(|&line| line.chars().collect()).collect()
}

fn walk(area: &Vec<Vec<char>>) -> (usize, Vec<(usize, usize)>)  {
    let mut cur_position = get_initial_position(area);
    let mut prev_position = cur_position;
    let mut walked_positions: Vec<(usize, usize)> = Vec::new();
    let mut dir = 0; // 0 - top, 1 - right, 2 - bottom, 3 - left
    while (cur_position.0 >= 0 && cur_position.0 < area.first().unwrap().len() as i32)
        && (cur_position.1 >= 0 && cur_position.1 < area.len() as i32) {
        let cur_char = area.get(cur_position.1 as usize).unwrap().get(cur_position.0 as usize).unwrap();
        if cur_char == &'#' {
            dir += 1;
            if dir == 4 {
                dir = 0;
            }
            cur_position = prev_position;
        }
        prev_position = cur_position;
        walked_positions.push((cur_position.0 as usize, cur_position.1 as usize));
        if dir == 0 {
            cur_position.1 -= 1;
        } else if dir == 1 {
            cur_position.0 += 1
        } else if dir == 2 {
            cur_position.1 += 1
        } else if dir == 3 {
            cur_position.0 -= 1
        }
    }
    let mut uq_positions: HashSet<(usize, usize)> = HashSet::new();
    for wp in &walked_positions {
        uq_positions.insert(*wp);
    }
    (uq_positions.len(), walked_positions)
}

fn walk_in_loops(area: &Vec<Vec<char>>, visited_positions: &Vec<(usize, usize)>) -> usize {
    let mut obstacles_count: usize = 0;
    for y in 0..area.len() {
        for x in 0..area.first().unwrap().len() {
            if !visited_positions.contains(&(x, y)) {
                continue;
            }
            let mut cur_position = get_initial_position(area);
            let mut prev_position = cur_position;
            let mut dir: usize = 0;
            let mut walked_positions_with_dir: Vec<(usize, usize, usize)> = Vec::new();
            while ((cur_position.0 >= 0 && cur_position.0 < area.first().unwrap().len() as i32)
                && (cur_position.1 >= 0 && cur_position.1 < area.len() as i32)) &&
                !walked_positions_with_dir.contains(&(cur_position.0 as usize, cur_position.1 as usize, dir)) {

                let cur_char = area.get(cur_position.1 as usize).unwrap().get(cur_position.0 as usize).unwrap();
                if cur_char == &'#' || (cur_position.0 as usize == x && cur_position.1 as usize == y)  {
                    dir += 1;
                    if dir == 4 {
                        dir = 0;
                    }
                    cur_position = prev_position;
                }
                prev_position = cur_position;
                walked_positions_with_dir.push((cur_position.0 as usize, cur_position.1 as usize, dir));
                if dir == 0 {
                    cur_position.1 -= 1;
                } else if dir == 1 {
                    cur_position.0 += 1
                } else if dir == 2 {
                    cur_position.1 += 1
                } else if dir == 3 {
                    cur_position.0 -= 1
                }
            }
            if walked_positions_with_dir.contains(&(cur_position.0 as usize, cur_position.1 as usize, dir)) {
                obstacles_count += 1;
            }
        }
    }
    obstacles_count
}

fn get_initial_position(area: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..area.len() {
        for x in 0..area[y].len() {
            if area[y][x] == '^' {
                return (x as i32, y as i32);
            }
        }
    }
    (0,0)
}