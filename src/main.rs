use std::{alloc::Layout, arch::x86_64::__m256i};

mod matrix;

#[cfg(test)]
mod test {
    use super::matrix::Matrix;
    use rand::random;

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
    fn test_push_pop() {
        let mut my_matrix: Matrix<f32> = Matrix::new();

        my_matrix.push_column(vec![2.0, 34.2]);
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2]]);

        my_matrix.push_column(vec![4.6, 6.4]);
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2], vec![4.6, 6.4]]);

        my_matrix.push_row(vec![5.7, 9.5]);
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2, 5.7], vec![4.6, 6.4, 9.5]]);

        assert_eq!(my_matrix.pop_columns(), Some(vec![4.6, 6.4, 9.5]));
        assert_eq!(my_matrix.as_nested_vec_ref(), &vec![vec![2.0, 34.2, 5.7]]);

        assert_eq!(my_matrix.pop_rows(), Some(vec![5.7]));
        assert_eq!(my_matrix.as_nested_vec(), vec![vec![2.0, 34.2]])
    }

    #[test]
    fn test_dot() {
        let my_matrix = Matrix::from(vec![vec![1f32, 4f32, 6f32], vec![3f32, 5f32, 8f32]]);
        let another = Matrix::from(vec![vec![3f32, 2f32], vec![5f32, 3f32], vec![7f32, 8f32]]);

        let result = my_matrix.dot(&another);

        println!("{:?}", result);

        assert_eq!(result.as_nested_vec(), vec![vec![9f32, 22f32, 34f32], vec![14f32, 35f32, 54f32], vec![31f32, 68f32, 106f32]])
    }

    #[test]
    fn test_random() {
        let mat = Matrix::<u8>::legacy_random::<f32>(30, 35, &|_| random::<u8>()%30, &|| 2.0);
        println!("{:?}", mat);
        assert_eq!(mat.len_cols(), 35);
        assert_eq!(mat.len_rows(), 30);
        assert_eq!(Matrix::valid_from_vector(mat.as_nested_vec_ref()).unwrap(), 30);
    }
}

fn main() {
    println!("Hello, world!");
}
