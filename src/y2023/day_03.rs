use crate::common::read_file_to_lines;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
struct Elem<'a, T> {
    matrix: &'a Matrix<T>,
    inner: T,
    row: usize,
    col: usize,
}

impl<'a, T> PartialEq for Elem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl<'a, T> Eq for Elem<'a, T> {}

impl<'a, T> Hash for Elem<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.row);
        state.write_usize(self.col);
    }
}

impl<'a, T> Clone for Elem<'a, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Elem {
            matrix: self.matrix,
            inner: self.inner.clone(),
            row: self.row,
            col: self.col,
        }
    }
}

impl<'a, T> Elem<'a, T>
where
    T: Clone,
{
    pub fn get_at_offset(&self, row: isize, col: isize) -> Option<Elem<'a, T>> {
        let elem_row = self.row.checked_add_signed(row)?;
        let elem_col = self.col.checked_add_signed(col)?;
        self.matrix.try_get_at(elem_row, elem_col).and_then(|i| {
            Some(Elem {
                matrix: self.matrix,
                inner: i,
                row: elem_row,
                col: elem_col,
            })
        })
    }

    pub fn neighbours(&self) -> Vec<Elem<'a, T>> {
        let mut out = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                if let Some(e) = self.get_at_offset(i, j) {
                    out.push(e);
                }
            }
        }
        out
    }

    pub fn find_matching(
        &self,
        condition: impl FnOnce(T) -> bool + Copy,
        row_offset: isize,
        col_offset: isize,
    ) -> Elem<'a, T> {
        let mut curr_elem = self.clone();
        loop {
            let left = curr_elem.get_at_offset(row_offset, col_offset);
            if left.as_ref().is_some_and(|e| condition(e.inner.clone())) {
                curr_elem = left.unwrap();
            } else {
                break;
            }
        }
        curr_elem
    }

    pub fn find_left_matching(&self, condition: impl FnOnce(T) -> bool + Copy) -> Elem<'a, T> {
        self.find_matching(condition, 0, -1)
    }

    pub fn find_right_matching(&self, condition: impl FnOnce(T) -> bool + Copy) -> Elem<'a, T> {
        self.find_matching(condition, 0, 1)
    }
}

impl<'a> Elem<'a, char> {
    pub fn string_right_matching(&self, condition: impl FnOnce(char) -> bool + Copy) -> String {
        let start = self.col;
        let end = self.find_right_matching(condition).col;
        let mut out = String::new();
        for i in start..end + 1 {
            out.push(self.matrix.get_at(self.row, i));
        }
        println!("456 {}, {}, {}", out, start, end);
        out
    }
}

impl<'a, T> std::ops::Deref for Elem<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

struct Matrix<T> {
    data: Vec<Vec<T>>,
    cols: usize,
    rows: usize,
}

impl<T> std::fmt::Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix")
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .finish()
    }
}

impl Matrix<char> {
    pub fn new(path: &str) -> Self {
        let mut data = Vec::new();
        let lines = read_file_to_lines(path);
        let cols = lines[0].len();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            data.push(line.chars().collect());
        }
        let rows = data.len();
        Matrix { data, cols, rows }
    }
}

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn col_count(&self) -> usize {
        self.cols
    }

    pub fn row_count(&self) -> usize {
        self.rows
    }

    pub fn try_get_at(&self, row: usize, col: usize) -> Option<T> {
        self.data.get(row).and_then(|r| r.get(col)).cloned()
    }

    pub fn get_at(&self, row: usize, col: usize) -> T {
        self.data[row][col].clone()
    }

    pub fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator {
            matrix: self,
            curr_row: 0,
            curr_col: 0,
        }
    }
}

struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    curr_row: usize,
    curr_col: usize,
}

impl<'a, T> Iterator for MatrixIterator<'a, T>
where
    T: Clone,
{
    type Item = Elem<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col >= self.matrix.col_count() {
            self.curr_row += 1;
            self.curr_col = 0;
        }
        if self.curr_row >= self.matrix.row_count() {
            return None;
        }
        let result = Elem {
            matrix: self.matrix,
            inner: self.matrix.get_at(self.curr_row, self.curr_col),
            row: self.curr_row,
            col: self.curr_col,
        };
        self.curr_col += 1;
        Some(result)
    }
}

pub fn run_1() {
    let matrix = Matrix::new("src/y2023/day_03.input");
    let mut num_pos: HashSet<Elem<char>> = HashSet::new();
    for elem in matrix.iter() {
        if elem.is_numeric() {
            if elem
                .neighbours()
                .iter()
                .any(|e| !e.is_numeric() && **e != '.')
            {
                num_pos.insert(elem.find_left_matching(char::is_numeric));
            }
        }
    }

    let mut sum = 0;
    for elem in num_pos.drain() {
        let number = elem.string_right_matching(char::is_numeric);
        sum += number.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

pub fn run_2() {
    let matrix = Matrix::new("src/y2023/day_03.input");
    let mut sum = 0;
    for elem in matrix.iter() {
        if elem.is_numeric() || *elem == '.' {
            continue;
        }
        let neighbors: Vec<(isize, isize)> = elem
            .neighbours()
            .iter()
            .filter(|e| e.is_numeric())
            .map(|e| {
                (
                    e.row as isize - elem.row as isize,
                    e.col as isize - elem.col as isize,
                )
            })
            .collect();
        let top_number_count = match neighbors.iter().filter(|e| e.0 == -1).count() {
            0 => 0,
            1 | 3 => 1,
            2 => {
                if elem.get_at_offset(-1, 0).is_some_and(|e| *e == '.') {
                    2
                } else {
                    1
                }
            }
            _ => panic!(),
        };
        let middle_number_count = match neighbors.iter().filter(|e| e.0 == 0).count() {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => panic!(),
        };
        let bottom_number_count = match neighbors.iter().filter(|e| e.0 == 1).count() {
            0 => 0,
            1 | 3 => 1,
            2 => {
                if elem.get_at_offset(1, 0).is_some_and(|e| *e == '.') {
                    2
                } else {
                    1
                }
            }
            _ => panic!(),
        };
        if top_number_count + middle_number_count + bottom_number_count != 2 {
            continue;
        }
        let mut num_pos: HashSet<Elem<char>> = HashSet::new();
        for off in neighbors.iter() {
            num_pos.insert(
                elem.get_at_offset(off.0, off.1)
                    .unwrap()
                    .find_left_matching(char::is_numeric),
            );
        }
        sum += num_pos
            .drain()
            .map(|e| e.string_right_matching(char::is_numeric))
            .map(|s| s.parse::<i32>().unwrap())
            .reduce(|acc, e| if acc == 0 { e } else { acc * e })
            .unwrap();
    }
    println!("{}", sum);
}
