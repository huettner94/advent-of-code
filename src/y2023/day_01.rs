use crate::common::read_file_to_lines;

pub fn run_1() {
    let lines = read_file_to_lines("src/y2023/day_01.input");
    let mut accum = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let first_char = line.chars().filter(|c| c.is_numeric()).next().unwrap();
        let last_char = line.chars().filter(|c| c.is_numeric()).last().unwrap();
        let numstring = format!("{}{}", first_char, last_char);
        accum += numstring.parse::<i32>().unwrap();
    }
    println!("{}", accum);
}

pub fn run_2() {
    let lines = read_file_to_lines("src/y2023/day_01.input");
    let mut accum = 0;
    let fakenum = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for mut line in lines {
        if line.is_empty() {
            continue;
        }
        let first_digit = line.find(char::is_numeric).unwrap();
        let mut firstidx = 0;
        let mut firstoffset = usize::MAX;
        let mut lastidx = 0;
        let mut lastoffset = 0;
        for i in 0..fakenum.len() {
            if let Some(off) = line.find(fakenum[i]) {
                if off < firstoffset && off < first_digit {
                    firstidx = i;
                    firstoffset = off
                }
            }
            if let Some(off) = line.rfind(fakenum[i]) {
                if off > lastoffset {
                    lastidx = i;
                    lastoffset = off
                }
            }
        }
        if firstoffset != usize::MAX {
            line = line.replacen(fakenum[firstidx], &(firstidx + 1).to_string(), 1);
        }
        if lastoffset != 0 {
            line = line.replace(fakenum[lastidx], &(lastidx + 1).to_string()); // this is ok, since
                                                                               // the first one has
                                                                               // already been
                                                                               // replaced
        }
        let first_char = line.chars().filter(|c| c.is_numeric()).next().unwrap();
        let last_char = line.chars().filter(|c| c.is_numeric()).last().unwrap();
        let numstring = format!("{}{}", first_char, last_char);
        accum += numstring.parse::<i32>().unwrap();
    }
    println!("{}", accum);
}
