use crate::common::read_file_to_lines;

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse_headers(header: &[String]) -> Self {
        let count = (header.last().unwrap().chars().count() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = (0..count).map(|_| Vec::new()).collect();
        for line in header.iter().rev().skip(1) {
            for idx in 0..count {
                let c = line.chars().nth(idx * 4 + 1).unwrap();
                if !c.is_whitespace() {
                    stacks[idx].push(c);
                }
            }
        }
        Stacks { stacks }
    }

    fn run_command_v1(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            let v = self.stacks[from].pop().unwrap();
            self.stacks[to].push(v);
        }
    }

    fn run_command_v2(&mut self, amount: usize, from: usize, to: usize) {
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..amount {
            tmp.push(self.stacks[from].pop().unwrap());
        }
        for _ in 0..amount {
            self.stacks[to].push(tmp.pop().unwrap());
        }
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_05.input");
    let (header, body) = lines.split_at(lines.iter().position(|e| e.is_empty()).unwrap());
    let mut stacks = Stacks::parse_headers(header);
    for line in body {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let amount = parts[1].parse().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;
        stacks.run_command_v1(amount, from, to);
    }
    for v in &stacks.stacks {
        print!("{}", v.last().unwrap());
    }
    println!();
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_05.input");
    let (header, body) = lines.split_at(lines.iter().position(|e| e.is_empty()).unwrap());
    let mut stacks = Stacks::parse_headers(header);
    for line in body {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        let amount = parts[1].parse().unwrap();
        let from = parts[3].parse::<usize>().unwrap() - 1;
        let to = parts[5].parse::<usize>().unwrap() - 1;
        stacks.run_command_v2(amount, from, to);
    }
    for v in &stacks.stacks {
        print!("{}", v.last().unwrap());
    }
    println!();
}
