pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = vec![0, 0];
    for i in 0..mat.len() {
        let ones = mat[i].iter().filter(|&&x| x == 1).count() as i32;
        if ones > result[1] {
            result[1] = ones;
            result[0] = i as i32;
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
            vec![0, 1],
            row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]])
        );
        assert_eq!(
            vec![1, 2],
            row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]])
        );
        assert_eq!(
            vec![1, 2],
            row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]])
        );
    }
}
