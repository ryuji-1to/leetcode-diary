pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        );
        assert_eq!(
            vec![vec![1, 4], vec![2, 5], vec![3, 6]],
            transpose(vec![vec![1, 2, 3], vec![4, 5, 6]])
        );
    }
}
