pub fn all_cells_dist_order(rows: i32, cols: i32, r_center: i32, c_center: i32) -> Vec<Vec<i32>> {
    let mut cordinates = Vec::with_capacity((rows * cols) as usize);
    for i in 0..rows {
        for j in 0..cols {
            cordinates.push(vec![i, j]);
        }
    }
    cordinates.sort_unstable_by_key(|a| a[0].abs_diff(r_center) + a[1].abs_diff(c_center));
    cordinates
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![0, 0], vec![0, 1]],
            all_cells_dist_order(1, 2, 0, 0)
        );
        assert_eq!(
            vec![vec![0, 1], vec![0, 0], vec![1, 1], vec![1, 0]],
            all_cells_dist_order(2, 2, 0, 1)
        );
        assert_eq!(
            vec![
                vec![1, 2],
                vec![0, 2],
                vec![1, 1],
                vec![0, 1],
                vec![1, 0],
                vec![0, 0]
            ],
            all_cells_dist_order(2, 3, 1, 2)
        );
    }
}
