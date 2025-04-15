pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut a = mat
        .iter()
        .map(|xs| xs.iter().filter(|&&x| x == 1).count())
        .enumerate()
        .collect::<Vec<_>>();
    a.sort_by_key(|&x| x.1);
    (0..k as usize).map(|i| a[i].0 as i32).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            ),
            vec![2, 0, 3]
        );

        assert_eq!(
            k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            ),
            vec![0, 2]
        );
    }
}
