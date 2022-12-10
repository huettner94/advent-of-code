use std::collections::HashSet;

use crate::common::read_file_to_lines;

fn to_prio(c: &char) -> i32 {
    let code = (*c as u8) as i32;
    if code >= 97 {
        code - 96
    } else {
        code - 64 + 26
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_03.input");
    let mut elems: Vec<char> = Vec::new();
    for line in lines {
        let (p1, p2) = line.split_at(line.len() / 2);
        let s1: HashSet<char> = p1.chars().collect();
        let s2: HashSet<char> = p2.chars().collect();
        let r: HashSet<&char> = s1.intersection(&s2).collect();
        elems.push(**r.iter().next().unwrap());
    }
    let sum: i32 = elems.iter().map(to_prio).sum();
    println!("sum {}", sum);
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_03.input");
    let mut elems: Vec<char> = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        let s1: HashSet<char> = lines[i].chars().collect();
        let s2: HashSet<char> = lines[i + 1].chars().collect();
        let s3: HashSet<char> = lines[i + 2].chars().collect();
        let r: HashSet<char> = s1.intersection(&s2).cloned().collect();
        let r: HashSet<&char> = r.intersection(&s3).collect();
        elems.push(**r.iter().next().unwrap());
    }
    let sum: i32 = elems.iter().map(to_prio).sum();
    println!("sum {}", sum);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_prio() {
        assert_eq!(to_prio(&'p'), 16);
        assert_eq!(to_prio(&'L'), 38);
        assert_eq!(to_prio(&'P'), 42);
    }
}
