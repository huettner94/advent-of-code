use crate::common::read_file_to_lines;

pub fn run_1() {
    let lines = read_file_to_lines("src/y2022/day_01.input");
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

pub fn run_2() {
    let lines = read_file_to_lines("src/y2022/day_01.input");
    let mut cal_per_elf: Vec<i32> = Vec::new();
    let mut accum = 0;
    for line in lines {
        if line.is_empty() {
            cal_per_elf.push(accum);
            accum = 0;
        } else {
            accum += line.parse::<i32>().unwrap();
        }
    }
    cal_per_elf.sort_unstable();
    cal_per_elf.reverse();
    let out: i32 = cal_per_elf[..3].iter().sum();
    println!("{}", out)
}
