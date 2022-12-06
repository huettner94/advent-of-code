use std::env::args;

mod common;
mod day_01;
mod day_02;

fn main() {
    let args: Vec<String> = args().collect();
    match &args[1][..] {
        "day01_1" => day_01::run_1(),
        "day01_2" => day_01::run_2(),
        "day02_1" => day_02::run_1(),
        "day02_2" => day_02::run_2(),
        _ => panic!("nope"),
    }
}
