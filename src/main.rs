use std::env::args;

mod common;
mod day_01;

fn main() {
    let args: Vec<String> = args().collect();
    match &args[1][..] {
        "day01_1" => day_01::run_1(),
        "day01_2" => day_01::run_2(),
        _ => panic!("nope"),
    }
}
