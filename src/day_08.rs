use crate::common::read_file_to_lines;

#[derive(Debug)]
struct Trees {
    data: Vec<Vec<u8>>,
    visible_top: Vec<Vec<bool>>,
    visible_left: Vec<Vec<bool>>,
    visible_right: Vec<Vec<bool>>,
    visible_bottom: Vec<Vec<bool>>,
}

impl Trees {
    fn from_lines(lines: &[String]) -> Self {
        let data = lines
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        Trees {
            data,
            visible_top: Vec::new(),
            visible_left: Vec::new(),
            visible_right: Vec::new(),
            visible_bottom: Vec::new(),
        }
    }

    fn calc_visibility_top(&mut self) {
        let mut max: Vec<u8> = Vec::with_capacity(self.data.len());
        for (i, row) in self.data.iter().enumerate() {
            for j in 0..max.len() {
                if row[j] > max[j] {
                    max[j] = row[j];
                    self.visible_top[i][j] = true;
                }
            }
        }
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_08.input");
    let trees = Trees::from_lines(&lines);
    println!("{:?}", trees);
}

pub fn run_2() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_overlaps() {}
}
