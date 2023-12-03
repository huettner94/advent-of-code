use crate::common::read_file_to_lines;
use std::cmp::max;

struct Cubes {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

struct Game {
    pub id: i32,
    pub rounds: Vec<Cubes>
}

fn load() -> Vec<Game> {
    let mut out = Vec::new();
    let lines = read_file_to_lines("src/y2023/day_02.input");
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (game, rounds) = line.split_once(':').unwrap();
        let gameid = game.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        let rounds: Vec<Cubes> = rounds.split("; ").map(|round| {
            let elem = round.split(", ");
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for e in elem {
                let (count, color) = e.trim().split_once(' ').unwrap();
                let count = count.parse::<i32>().unwrap();
                match color {
                    "red" => red = count,
                    "green" => green = count,
                    "blue" => blue = count,
                    _ => panic!()
                }
            }
            Cubes{red, green, blue}
        }).collect();
        out.push(Game{id: gameid, rounds});
    }
    out
}

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

pub fn run_1() {
    let games = load();
    let mut possible_count = 0;
    for game in games {
        let mut possible = true;
        for round in game.rounds {
            if round.red > MAX_RED || round.green > MAX_GREEN || round.blue > MAX_BLUE {
                possible = false;
            }
        }
        if possible {
            possible_count += game.id;
        }
    }
    println!("{}", possible_count);
}

pub fn run_2() {
    let games = load();
    let mut power = 0;
    for game in games {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for round in game.rounds {
            min_red = max(min_red, round.red);
            min_green = max(min_green, round.green);
            min_blue = max(min_blue, round.blue);
        }
        power += min_red * min_green * min_blue;
    }
    println!("{}", power);
}
