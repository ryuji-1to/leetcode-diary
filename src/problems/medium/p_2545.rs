pub fn sort_the_students(mut score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    score.sort_unstable_by_key(|x| -x[k as usize]);
    score
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![7, 5, 11, 2], vec![10, 6, 9, 1], vec![4, 8, 3, 15]],
            sort_the_students(
                vec![vec![10, 6, 9, 1], vec![7, 5, 11, 2], vec![4, 8, 3, 15]],
                2
            )
        );
        assert_eq!(
            vec![vec![5, 6], vec![3, 4]],
            sort_the_students(vec![vec![3, 4], vec![5, 6]], 0)
        );
    }
}
