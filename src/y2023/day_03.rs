use crate::common::read_file_to_lines;
use std::{collections::HashSet, hash::Hash};

#[derive(Debug)]
struct Elem<'a, T> {
    matrix: &'a Matrix<T>,
    inner: T,
    row: usize,
    col: usize
}

impl<'a, T> PartialEq for Elem<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl<'a, T> Eq for Elem<'a, T> {
}

impl<'a, T> Hash for Elem<'a, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.row);
        state.write_usize(self.col);
    }
}

impl<'a, T> Elem<'a, T> 
where
    T: Clone,
{
    pub fn get_at_offset(&self, row: isize, col: isize) -> Option<Elem<'a, T>> {
        let elem_row = self.row.checked_add_signed(row)?;
        let elem_col = self.col.checked_add_signed(col)?;
        self.matrix.try_get_at(elem_row, elem_col).and_then(|i| Some(Elem {matrix: self.matrix, inner: i, row: elem_row, col: elem_col}))
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
        f.debug_struct("Matrix").field("cols", &self.cols).field("rows", &self.rows).finish()
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
        MatrixIterator { matrix: self, curr_row: 0, curr_col: 0 }
    }
}

struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    curr_row: usize,
    curr_col: usize
}

impl<'a, T> Iterator for MatrixIterator<'a, T> 
where
    T: Clone
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
        let result = Elem {matrix: self.matrix, inner: self.matrix.get_at(self.curr_row, self.curr_col), row: self.curr_row, col: self.curr_col};
        self.curr_col += 1;
        Some(result)
    }
}


pub fn run_1() {
    let matrix = Matrix::new("src/y2023/day_03.input");
    let mut num_pos: HashSet<Elem<char>> = HashSet::new();
    for elem in matrix.iter() {
        if elem.is_numeric() {
            if elem.neighbours().iter().any(|e| !e.is_numeric() && **e != '.') {
                let mut curr_elem = elem;
                loop {
                    let left = curr_elem.get_at_offset(0, -1);
                    if left.as_ref().is_some_and(|e| e.is_numeric()) {
                        curr_elem = left.unwrap();
                    } else {
                        break;
                    }
                }
                num_pos.insert(curr_elem);
            }
        }
    }

    let mut sum = 0;
    for mut elem in num_pos.drain() {
        let mut number = String::new();
        loop {
            number.push(*elem);
            let right = elem.get_at_offset(0, 1);
            if right.as_ref().is_some_and(|e| e.is_numeric()) {
                elem = right.unwrap();
            } else {
                break;
            }
        }
        sum += number.parse::<i32>().unwrap();
    }
    println!("{}", sum);
}

pub fn run_2() {
}
