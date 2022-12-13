use std::collections::HashSet;

use itertools::Itertools;

use crate::common::read_file_to_lines;

pub fn run_1() {
    let line = &read_file_to_lines("src/day_06.input")[0];
    for ((_, e1), (_, e2), (_, e3), (idx, e4)) in line.chars().enumerate().tuples() {
        let mut s: HashSet<char> = HashSet::new();
        s.insert(e1);
        s.insert(e2);
        s.insert(e3);
        s.insert(e4);
        if s.len() == 4 {
            println!("{}", idx + 1);
            break;
        }
    }
    for i in 4..line.len() {
        let mut s: HashSet<char> = HashSet::new();
        s.extend(line.chars().skip(i - 4).take(4));
        if s.len() == 4 {
            println!("{}", i);
            break;
        }
    }
}

pub fn run_2() {
    let line = &read_file_to_lines("src/day_06.input")[0];
    for i in 14..line.len() {
        let mut s: HashSet<char> = HashSet::new();
        s.extend(line.chars().skip(i - 14).take(14));
        if s.len() == 14 {
            println!("{}", i);
            break;
        }
    }
}
