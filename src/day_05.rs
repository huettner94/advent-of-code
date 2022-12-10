use crate::common::read_file_to_lines;

struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn parse_headers(header: Vec<String>) -> Self {
        let count = header.last().unwrap().split(' ').count();
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_05.input");
}

pub fn run_2() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_overlaps() {}
}
