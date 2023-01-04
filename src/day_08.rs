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
        let data: Vec<Vec<u8>> = lines
            .iter()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let rows = data.len();
        let cols = data[0].len();
        let mut me = Trees {
            data,
            visible_top: vec![vec![false; cols]; rows],
            visible_left: vec![vec![false; cols]; rows],
            visible_right: vec![vec![false; cols]; rows],
            visible_bottom: vec![vec![false; cols]; rows],
        };
        me.calc_visibility();
        me
    }

    fn count_visible(&self) -> usize {
        let visible = self.get_visibility();
        let mut out = 0;
        for l in visible {
            for e in l {
                if e {
                    out += 1;
                } else {
                }
            }
            println!();
        }
        out
    }

    fn get_visibility(&self) -> Vec<Vec<bool>> {
        let rows = self.data.len();
        let cols = self.data[0].len();
        let mut visible = vec![vec![false; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                visible[i][j] = self.visible_top[i][j]
                    | self.visible_bottom[i][j]
                    | self.visible_left[i][j]
                    | self.visible_right[i][j];
            }
        }
        visible
    }

    fn calc_visibility_range_vert(
        data: &[Vec<u8>],
        writeable_vec: &mut [Vec<bool>],
        outer: Vec<usize>,
        inner: Vec<usize>,
    ) {
        let mut max: Vec<i32> = vec![-1; inner.len()];
        for i in outer {
            for j in inner.clone() {
                let val = data[i][j];
                if val as i32 > max[j] {
                    max[j] = val as i32;
                    writeable_vec[i][j] = true;
                }
            }
        }
    }

    fn calc_visibility_range_horiz(
        data: &[Vec<u8>],
        writeable_vec: &mut [Vec<bool>],
        outer: Vec<usize>,
        inner: Vec<usize>,
    ) {
        let mut max: Vec<i32> = vec![-1; inner.len()];
        for j in outer {
            for i in inner.clone() {
                let val = data[i][j];
                if val as i32 > max[i] {
                    max[i] = val as i32;
                    writeable_vec[i][j] = true;
                }
            }
        }
    }

    fn calc_visibility(&mut self) {
        Trees::calc_visibility_range_vert(
            &self.data,
            &mut self.visible_top,
            (0..self.data.len()).collect(),
            (0..self.data[0].len()).collect(),
        );
        Trees::calc_visibility_range_vert(
            &self.data,
            &mut self.visible_bottom,
            (0..self.data.len()).rev().collect(),
            (0..self.data[0].len()).collect(),
        );
        Trees::calc_visibility_range_horiz(
            &self.data,
            &mut self.visible_left,
            (0..self.data[0].len()).collect(),
            (0..self.data.len()).collect(),
        );
        Trees::calc_visibility_range_horiz(
            &self.data,
            &mut self.visible_right,
            (0..self.data[0].len()).rev().collect(),
            (0..self.data.len()).collect(),
        );
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> u64 {
        let mut score: u64 = 0;
        let mut dir_score: u64 = 0;
        let self_size = self.data[x][y];
        for i in (0..y).rev() {
            dir_score += 1;
            if self.data[x][i] >= self_size {
                break;
            }
        }
        score = dir_score;
        dir_score = 0;
        for i in (y + 1)..self.data[x].len() {
            dir_score += 1;
            if self.data[x][i] >= self_size {
                break;
            }
        }
        if dir_score > 0 {
            if score == 0 {
                score = dir_score;
            } else {
                score *= dir_score;
            }
        }
        dir_score = 0;
        for i in (0..x).rev() {
            dir_score += 1;
            if self.data[i][y] >= self_size {
                break;
            }
        }
        if dir_score > 0 {
            if score == 0 {
                score = dir_score;
            } else {
                score *= dir_score;
            }
        }
        dir_score = 0;
        for i in (x + 1)..self.data.len() {
            dir_score += 1;
            if self.data[i][y] >= self_size {
                break;
            }
        }
        if dir_score > 0 {
            if score == 0 {
                score = dir_score;
            } else {
                score *= dir_score;
            }
        }
        score
    }

    fn get_scenic_scores(&self) -> Vec<Vec<u64>> {
        let rows = self.data.len();
        let cols = self.data[0].len();
        let mut out = vec![vec![0; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                out[i][j] = self.get_scenic_score(i, j);
                print!(" {}", out[i][j]);
            }
            println!();
        }
        out
    }

    fn get_max_scenic_score(&self) -> u64 {
        *self.get_scenic_scores().iter().flatten().max().unwrap()
    }
}

pub fn run_1() {
    let lines = read_file_to_lines("src/day_08.input");
    let trees = Trees::from_lines(&lines);
    println!("{:?}", trees.count_visible());
}

pub fn run_2() {
    let lines = read_file_to_lines("src/day_08.input");
    let trees = Trees::from_lines(&lines);
    //println!("{}", trees.get_scenic_score(trees.data.len() - 1, 0));
    println!("{:?}", trees.get_max_scenic_score());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scenic_scores() {
        let lines = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        let trees = Trees::from_lines(&lines);
        let scores = trees.get_scenic_scores();
        assert_eq!(scores[0], vec![4, 1, 2, 12, 3]);
    }
}
