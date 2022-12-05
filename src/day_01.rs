use crate::common::read_file_to_lines;

pub fn run_1() {
    let lines = read_file_to_lines("src/day_01.input");
    let mut max = 0;
    let mut accum = 0;
    for line in lines {
        if line.is_empty() {
            if accum > max {
                max = accum;
            }
            accum = 0;
        } else {
            accum += line.parse::<i32>().unwrap();
        }
    }
    println!("{}", max);
}

pub fn run_2() {}
