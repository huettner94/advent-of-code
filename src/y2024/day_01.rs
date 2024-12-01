use crate::common::read_file_to_lines;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    let lines = read_file_to_lines("src/y2024/day_01.input");
    let mut left = Vec::with_capacity(lines.len());
    let mut right = Vec::with_capacity(lines.len());
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    }
    left.sort_unstable();
    right.sort_unstable();
    assert_eq!(left.len(), right.len());
    (left, right)
}

pub fn run_1() {
    let (left, right) = read_input();
    let mut distance = 0;
    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs()
    }
    println!("{}", distance);
}

pub fn run_2() {
    let (left, right) = read_input();
    let mut similarity = 0;
    for i in 0..left.len() {
        similarity += left[i] * right.iter().filter(|e| **e == left[i]).count() as i32;
    }
    println!("{}", similarity);
}
