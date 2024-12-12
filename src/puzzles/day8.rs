use std::collections::HashSet;
use std::fs;

pub fn solve() {
    let input = parse_input();
    let result_p1 = calculate_antinodes(&input);
    println!("{:?}", result_p1);
    let result_p2 = calculate_extended_antinodes(&input);
    println!("{:?}", result_p2)
}

fn parse_input() -> Vec<Vec<char>> {
    let input = fs::read_to_string(&"src/input/day8.txt".to_string()).unwrap();
    let lines : Vec<&str> = input.lines().collect();
    lines.iter().map(|&line| line.chars().collect()).collect()
}

fn calculate_antinodes(grid: &Vec<Vec<char>>) -> usize {
    let mut result = HashSet::new();
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            let ch = grid[y][x];
            if ch != '.' {
                let neighbors = get_neighbors(grid, (x, y), ch);
                let antinodes = get_antinodes((x as i32, y as i32), &neighbors, (width - 1) as i32, (height - 1) as i32);
                for a in antinodes {
                    result.insert(a);
                }
            }
        }
    }
    result.len()
}

fn calculate_extended_antinodes(grid: &Vec<Vec<char>>) -> usize {
    let mut result = HashSet::new();
    let height = grid.len();
    let width = grid[0].len();
    for y in 0..height {
        for x in 0..width {
            let ch = grid[y][x];
            if ch != '.' {
                let neighbors = get_neighbors(grid, (x, y), ch);
                let antinodes = get_antinodes((x as i32, y as i32), &neighbors, (width - 1) as i32, (height - 1) as i32);
                //println!("node: {:?}, antinodes: {:?}", (x, y), antinodes);
                let mut extended_antinodes: HashSet<(i32, i32)> = HashSet::new();
                for a in &antinodes {
                    extended_antinodes.insert((a.0, a.1));
                }
                for a in &antinodes {
                    let mut new_antinodes: Vec<(i32, i32)> = Vec::new();
                    let a_antinodes = get_next_antinode(*a, (x as i32, y as i32), &mut new_antinodes, (width - 1) as i32, (height - 1) as i32);
                    //println!("antinode: {:?}, antinodes: {:?}", a, a_antinodes);

                    for a in &a_antinodes {
                        extended_antinodes.insert((a.0, a.1));
                    }
                }
                result.insert((x as i32, y as i32));
                result.extend(extended_antinodes);
            }
        }
    }
    //println!("all extended antinodes: {:?}", result);
    result.len()
}

fn get_neighbors(grid: &Vec<Vec<char>>, node: (usize, usize), ch: char) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == ch && node.0 != x && node.1 != y {
                neighbors.push((x as i32, y as i32));
            }
        }
    }
    neighbors
}

fn get_antinodes(node: (i32, i32), neighbors: &Vec<(i32, i32)>, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for (x, y) in neighbors {
        let dist = ((node.0 - x), (node.1 - y));
        let antinode = (node.0 + dist.0, node.1 + dist.1);
        if antinode.0 < 0 || antinode.1 < 0 || antinode.0 > max_y || antinode.1 > max_x {
            continue
        }
        result.push((antinode.0, antinode.1))
    }
    result
}

fn get_next_antinode(cur: (i32, i32), prev: (i32, i32), acc: &mut Vec<(i32, i32)>, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    if cur.0 < 0 || cur.1 < 0 || cur.0 > max_x || cur.1 > max_y {
        return acc.clone()
    }
    let dist = ((cur.0 - prev.0), (cur.1 - prev.1));
    let new_cur = (cur.0 + dist.0, cur.1 + dist.1);
    acc.push(cur);
    get_next_antinode(new_cur, cur, acc, max_x, max_y)
}