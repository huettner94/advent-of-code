use crate::common::read_file_to_lines;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let (p1_1, p1_2) = s.split_once('-').unwrap();
        Range {
            start: p1_1.parse().unwrap(),
            end: p1_2.parse().unwrap(),
        }
    }

    fn is_superrange(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_inner(&self, other: &Range) -> bool {
        for i in self.start..self.end + 1 {
            if other.start <= i && other.end >= i {
                return true;
            }
        }
        false
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.overlaps_inner(other) || other.overlaps_inner(self)
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_04.input");
    let mut count = 0;
    for line in lines {
        let (p1, p2) = line.split_once(',').unwrap();
        let r1 = Range::from_str(p1);
        let r2 = Range::from_str(p2);
        if r1.is_superrange(&r2) || r2.is_superrange(&r1) {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_04.input");
    let mut count = 0;
    for line in lines {
        let (p1, p2) = line.split_once(',').unwrap();
        let r1 = Range::from_str(p1);
        let r2 = Range::from_str(p2);
        if r1.overlaps(&r2) {
            count += 1;
        }
    }
    println!("{}", count);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_overlaps() {
        assert!(!Range::from_str("2-4").overlaps(&Range::from_str("6-8")));
        assert!(!Range::from_str("2-3").overlaps(&Range::from_str("4-5")));
        assert!(Range::from_str("5-7").overlaps(&Range::from_str("7-9")));
        assert!(Range::from_str("2-8").overlaps(&Range::from_str("3-7")));
        assert!(Range::from_str("6-6").overlaps(&Range::from_str("4-6")));
        assert!(Range::from_str("2-6").overlaps(&Range::from_str("4-8")));
    }
}
