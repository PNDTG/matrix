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

#[cfg(test)]
mod test {
    use super::Matrix;

    #[test]
    fn test_init() {
        let my_matrix: Matrix<f32> = Matrix::new();
        let my_vector: Vec<Vec<f32>> = vec![Vec::new()];
        
        println!("{:?}", Matrix::valid_from_vector(&my_vector));
        let another_mat: Matrix<f32> = Matrix::from(my_vector);

        println!("my_mat -> {:?}, another one -> {:?}", my_matrix, another_mat);

        assert_eq!(another_mat.as_nested_vec(), vec![vec![]]);
    }

    #[test]
    fn test_push() {
        let mut my_matrix: Matrix<f32> = Matrix::new();

        my_matrix.push_column(vec![2.0, 34.2]);
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2]]);

        my_matrix.push_column(vec![4.6, 6.4]);
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2], vec![4.6, 6.4]]);

        my_matrix.push_row(vec![5.7, 9.5]);
        assert_eq!(my_matrix.as_nested_vec(), vec![vec![2.0, 34.2, 5.7], vec![4.6, 6.4, 9.5]]);
    }

    #[test]
    fn test_dot() {
        let my_matrix = Matrix::from(vec![vec![1f32, 4f32, 6f32], vec![3f32, 5f32, 8f32]]);
        let another = Matrix::from(vec![vec![3f32, 2f32], vec![5f32, 3f32], vec![7f32, 8f32]]);

        let result = my_matrix.dot(&another);

        println!("{:?}", result);

        assert_eq!(result.as_nested_vec(), vec![vec![9f32, 22f32, 34f32], vec![14f32, 35f32, 54f32], vec![31f32, 68f32, 106f32]])
    }
}
fn main() {
    println!("Hello, world!");
}
