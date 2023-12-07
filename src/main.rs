use std::env::args;

mod common;
mod y2022;
mod y2023;

fn main() {
    let args: Vec<String> = args().collect();
    match &args[1][..] {
        "2022" => match &args[2][..] {
            "day01_1" => y2022::day_01::run_1(),
            "day01_2" => y2022::day_01::run_2(),
            "day02_1" => y2022::day_02::run_1(),
            "day02_2" => y2022::day_02::run_2(),
            "day03_1" => y2022::day_03::run_1(),
            "day03_2" => y2022::day_03::run_2(),
            "day04_1" => y2022::day_04::run_1(),
            "day04_2" => y2022::day_04::run_2(),
            "day05_1" => y2022::day_05::run_1(),
            "day05_2" => y2022::day_05::run_2(),
            "day06_1" => y2022::day_06::run_1(),
            "day06_2" => y2022::day_06::run_2(),
            "day07_1" => y2022::day_07::run_1(),
            "day07_2" => y2022::day_07::run_2(),
            "day08_1" => y2022::day_08::run_1(),
            "day08_2" => y2022::day_08::run_2(),
            _ => panic!("nope"),
        },
        "2023" => match &args[2][..] {
            "day01_1" => y2023::day_01::run_1(),
            "day01_2" => y2023::day_01::run_2(),
            "day02_1" => y2023::day_02::run_1(),
            "day02_2" => y2023::day_02::run_2(),
            "day03_1" => y2023::day_03::run_1(),
            "day03_2" => y2023::day_03::run_2(),
            _ => panic!("nope"),
        },
        _ => panic!("nope"),
    }
}
