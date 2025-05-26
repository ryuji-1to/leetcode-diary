pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    if m * n != original.len() as i32 {
        return vec![];
    }
    original
        .chunks(n as usize)
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<i32>>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 4]],
            construct2_d_array(vec![1, 2, 3, 4], 2, 2)
        );
        assert_eq!(vec![vec![1, 2, 3]], construct2_d_array(vec![1, 2, 3], 1, 3));
        assert_eq!(
            vec![] as Vec<Vec<i32>>,
            construct2_d_array(vec![1, 2], 1, 1)
        );
    }
}
