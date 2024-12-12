use std::fs;
use std::char;
use std::path::Component::ParentDir;

pub fn solve() {
    let input = parse_input();
    println!("{:?}", shuffle_up_and_calculate(&input));
    println!("{:?}", shuffle_files_and_calculate(&input))
}

fn parse_input() -> Vec<i32> {
    let input = fs::read_to_string(&"src/input/day9.txt".to_string()).unwrap();
    input.chars().map(|c| c.to_string().parse().unwrap()).collect()
}

fn shuffle_up_and_calculate(input: &Vec<i32>) -> i64 {
    let mut result: i64 = 0;
    let mut disk_space = to_disk_space(input);
    shuffle_blocks(&mut disk_space);
    let mut idx = 0;
    for it in disk_space {
        if it == "." {
            break
        }
        let mn: i64 = it.parse::<i64>().unwrap() * idx;
        result += mn;
        idx += 1;
    }
    result
}

fn shuffle_files_and_calculate(input: &Vec<i32>) -> i64 {
    let mut result: i64 = 0;
    let mut disk_space = to_disk_space(input);
    shuffle_files(&mut disk_space);
    let mut idx = 0;
    for it in disk_space {
        if it == "." {
            idx += 1;
            continue;
        }
        let mn: i64 = it.parse::<i64>().unwrap() * idx;
        result += mn;
        idx += 1;
    }
    result
}

fn to_disk_space(input: &Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut is_file_info= true;
    let mut cur_idx = 0;
    for c in input {
        if is_file_info {
            let mut chars = vec![(cur_idx).to_string(); *c as usize];
            result.append(&mut chars);
            cur_idx += 1;
        } else {
            let mut chars = vec![".".to_string(); *c as usize];
            result.append(&mut chars);
        }
        is_file_info = !is_file_info;
    }
    result
}

fn shuffle_blocks(disk: &mut Vec<String>) {
    let mut last_idx = disk.len() - 1;
    let mut idx = 0;
    while last_idx > idx {
        if disk[idx] == "." {
            disk.swap(idx, last_idx);
            last_idx -= 1;
        } else {
            idx += 1;
        }
    }
}

fn shuffle_files(disk_space: &mut Vec<String>) {
    let v = disk_space.clone();
    let mut last_id = 0;
    let mut last_file_start = disk_space.len() - 1;
    for i in (0..disk_space.len()).rev() {
        if disk_space[i] != "." {
            last_id = disk_space[i].parse::<i64>().unwrap();
            last_file_start = i;
            break;
        }
    }
    let mut last_file_end = 0;
    let mut found_file= false;
    for i in (0..disk_space.len()).rev() {
        if disk_space[i] == "." || disk_space[i] != last_id.to_string() {
            if found_file {
                /// try move
                let mut start_free = 0;
                let mut end_free = 0;
                let mut started_free_space = false;
                for j in 0..disk_space.len() {
                    if (end_free - start_free) >= (last_file_end - last_file_start) && start_free != 0 && start_free < last_file_start {
                        while last_file_end >= last_file_start {
                            disk_space.swap(start_free, last_file_end);
                            start_free += 1;
                            last_file_end -= 1;
                        }
                        break;
                    }
                    if disk_space[j] == "." {
                        if started_free_space {
                            end_free = j;
                        } else {
                            start_free = j;
                            end_free = j;
                            started_free_space = true;
                        }
                    } else {
                        started_free_space = false;
                    }
                }
                found_file = false;
            }
            if disk_space[i] != "." && disk_space[i].parse::<i64>().unwrap() < last_id {
                last_id = disk_space[i].parse::<i64>().unwrap();
                last_file_end = i;
                last_file_start = i;
                found_file = true;
            }
        } else {
            if !found_file {
                last_id = disk_space[i].parse::<i64>().unwrap();
                found_file = true;
                last_file_end = i;
            }
            last_file_start = i;
        }
    }
}