#[derive(Debug)]
pub struct Matrix<T> {
    mat: Vec<Vec<T>>,
    rows: usize,
    columns: usize
}

impl<T> std::ops::Index<usize> for Matrix<T> {
    type Output = Vec<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.mat[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mat[index]
    }
}

impl<T: PartialEq> std::cmp::PartialEq for Matrix<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.columns != other.len_cols() || self.len_rows() != other.len_rows() {
            return false
        }
        for i in 0..self.columns {
            for j in 0..self.len_rows() {
                if self[i][j] != other[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl<T> IntoIterator for Matrix<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.mat.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Matrix<T> {
    type Item = &'a Vec<T>;
    type IntoIter = std::slice::Iter<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        let matrix = &self.mat;
        matrix.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Matrix<T> {
    type Item = &'a mut Vec<T>;
    type IntoIter = std::slice::IterMut<'a, Vec<T>>;

    fn into_iter(self) -> Self::IntoIter {
        let matrix = &mut self.mat;
        matrix.iter_mut()
    }
}

impl<T> Default for Matrix<T> {
    fn default() -> Self {
        Matrix { mat: Vec::from(Vec::new()), rows: 0, columns: 0 }
    }
}

impl<T> Matrix<T> {
    pub fn new() -> Self {
        Matrix { mat: Vec::from(Vec::new()), rows: 0, columns: 0 } // Not sure if I should return the Default trait or not
    }

    pub fn from(matrix: Vec<Vec<T>>) -> Self {
        let columns: usize = matrix.len();
        let rows: usize;
        match Matrix::valid_from_vector(&matrix) {
            Some(r) => rows = r,
            None => panic!("The size of all inner Vectors must be the same.")
        }
        Matrix { mat: matrix, rows: rows, columns: columns }
    }

    pub fn valid_from_vector(vector: &Vec<Vec<T>>) -> Option<usize> {
        let rows: usize;

        match vector.first() {
            Some(first) => rows = first.len(),
            None => return None
        }
        for i in vector {
            if i.len() != rows {
                return None;
            }
        }

        Some(rows)
    }

    pub fn push_row(&mut self, row: Vec<T>)
        where T: Copy {
        if self.columns != row.len() {
            panic!("Vector size must match Matrix.columns size!(Given row.len() -> {}, Given Matrix.columns -> {})",
                row.len(),
                self.columns)
        }
        for i in 0..self.columns {
            self.mat[i].push(row[i]);
            self.rows += 1;
        }
    }

    pub fn push_column(&mut self, column: Vec<T>) {
        if self.rows != column.len() && self.rows != 0 {
            panic!("Vector size must match Matrix.rows size!(Given column.len() -> {}, Given Matrix.rows -> {})",
                column.len(),
                self.rows)
        }
        self.mat.push(column);
        self.columns += 1;
    }

    // TODO: Remember to add pop, I totally forgot about it. (TEMP: ON IT! 3:48.19/2/24)
    pub fn pop_rows(&mut self) -> Option<Vec<T>> {
        let mut res: Vec<T> = Vec::new();
        if self.rows == 0 {return None}
        for i in &mut self.mat {
            res.push(i.pop().unwrap());
        }
        Some(res)
    }

    pub fn pop_columns(&mut self) -> Option<Vec<T>> {
        if self.columns == 0 {return None}
        Some(self.mat.pop().unwrap())
    }
    
    pub fn row(&self, row_idx: usize) -> Vec<T> 
        where T: Copy{
        let mut res: Vec<T> = Vec::new();
        for i in self {
            res.push(i[row_idx]);
        }
        res
    }

    pub fn column(&self, column_idx: usize) -> Vec<T> 
        where T: Copy{
        let mut res: Vec<T> = Vec::new();
        for i in &self[column_idx] {
            res.push(*i)
        }
        res
    }

    pub fn len_cols(&self) -> usize {
        self.columns
    }

    pub fn len_rows(&self) -> usize {
        self.rows
    }

    pub fn as_nested_vec(self) -> Vec<Vec<T>> {
        self.mat
    }

    pub fn as_nested_vec_ref(&self) -> &Vec<Vec<T>> {
        &self.mat
    }
}

impl<T: Default> Matrix<T> {
    pub fn zeros(rows: usize, columns: usize) -> Self 
        where T: Copy {
        Matrix { mat: vec![vec![T::default(); rows]; columns], rows: rows, columns: columns }
    }

    pub fn zeros_no_copy(rows: usize, columns: usize) -> Self {
        let mut mat: Vec<Vec<T>> = Vec::new();
        for _ in 0..columns {
            mat.push(Vec::new());
            for _ in 0..rows {
                mat.last_mut().unwrap().push(T::default());
            }
        }
        Matrix { mat: mat, rows: rows, columns: columns }
    }

    pub fn random<S>(rows: usize, columns: usize, rand_fn: &dyn Fn(S) -> T, seed_fn: &dyn Fn() -> S) -> Matrix<T> 
        where T: Copy{
        let mut mat = Matrix::zeros(rows, columns);

        for i in &mut mat {
            for j in i {
                *j = rand_fn(seed_fn());
            }
        }

        mat
    }
}

mod matrix_num {

    use std::ops::Mul;
    use std::ops::Add;
    use std::iter::Sum;
    use super::Matrix;
    impl<N: Copy + Mul<Output = N> + Add<N, Output = N> + Sum > Matrix<N> {
    
        pub fn dot(&self, matrix_2: &Self) -> Self 
            where N: Default{
            if self.columns != matrix_2.rows {
                panic!("Mismatched rows and columns, cannot perform dot product:\n
                    The columns of the first origin matrix and the rows of the second matrix must be the same size.\n
                    Given origin columns -> {}, Given second rows -> {}", self.columns, matrix_2.rows);
            }
            let mut mat: Self = Matrix::zeros(self.rows, matrix_2.columns);
    
            for i in 0..mat.len_cols() {
                for j in 0..mat.len_rows() {
                    mat[i][j] = Matrix::vector_dot(matrix_2.column(i), self.row(j))
                }
            }
    
            mat
        }
    
        pub fn vector_dot(vec_1: Vec<N>, vec_2: Vec<N>) -> N {
            vec_1.iter().zip(vec_2.iter()).map(|(x, y)| *x * *y).sum()
        }
    }
}