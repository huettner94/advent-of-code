use std::env::args;

mod common;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    let args: Vec<String> = args().collect();
    match &args[1][..] {
        "day01_1" => day_01::run_1(),
        "day01_2" => day_01::run_2(),
        "day02_1" => day_02::run_1(),
        "day02_2" => day_02::run_2(),
        "day03_1" => day_03::run_1(),
        "day03_2" => day_03::run_2(),
        "day04_1" => day_04::run_1(),
        "day04_2" => day_04::run_2(),
        "day05_1" => day_05::run_1(),
        "day05_2" => day_05::run_2(),
        _ => panic!("nope"),
    }
}
