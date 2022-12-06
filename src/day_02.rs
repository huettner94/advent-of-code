use crate::common::read_file_to_lines;

#[derive(PartialEq, Eq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scisors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<&str> for Choice {
    fn from(v: &str) -> Self {
        match v {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scisors,
            _ => panic!(),
        }
    }
}

impl From<&str> for Outcome {
    fn from(v: &str) -> Self {
        match v {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!(),
        }
    }
}

impl Choice {
    pub fn wins_over(&self, other: &Self) -> Outcome {
        match self {
            Choice::Rock => match other {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Lose,
                Choice::Scisors => Outcome::Win,
            },
            Choice::Paper => match other {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Draw,
                Choice::Scisors => Outcome::Lose,
            },
            Choice::Scisors => match other {
                Choice::Rock => Outcome::Lose,
                Choice::Paper => Outcome::Win,
                Choice::Scisors => Outcome::Draw,
            },
        }
    }

    pub fn get_points(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scisors => 3,
        }
    }

    pub fn what_to_choose(&self, outcome: &Outcome) -> Self {
        match self {
            Choice::Rock => match outcome {
                Outcome::Win => Self::Paper,
                Outcome::Draw => Self::Rock,
                Outcome::Lose => Self::Scisors,
            },
            Choice::Paper => match outcome {
                Outcome::Win => Self::Scisors,
                Outcome::Draw => Self::Paper,
                Outcome::Lose => Self::Rock,
            },
            Choice::Scisors => match outcome {
                Outcome::Win => Self::Rock,
                Outcome::Draw => Self::Scisors,
                Outcome::Lose => Self::Paper,
            },
        }
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_02.input");
    let mut score = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let theirs: Choice = parts[0].into();
        let mine: Choice = parts[1].into();
        score += mine.get_points();
        match mine.wins_over(&theirs) {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Lose => {}
        }
    }
    println!("{}", score);
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_02.input");
    let mut score = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let theirs: Choice = parts[0].into();
        let goal: Outcome = parts[1].into();
        let mine = theirs.what_to_choose(&goal);
        score += mine.get_points();
        match goal {
            Outcome::Win => score += 6,
            Outcome::Draw => score += 3,
            Outcome::Lose => {}
        }
    }
    println!("{}", score);
}
