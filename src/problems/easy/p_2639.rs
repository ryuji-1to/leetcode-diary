pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
    // let mut result = vec![0; grid[0].len()];
    // for g in grid {
    //     for i in 0..g.len() {
    //         let len = g[i].to_string().len() as i32;
    //         if len > result[i] {
    //             result[i] = len;
    //         }
    //     }
    // }
    // result
    (0..grid[0].len())
        .map(|col| {
            grid.iter()
                .map(|row| row[col].to_string().len() as i32)
                .max()
                .unwrap_or(0)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3],
            find_column_width(vec![vec![1], vec![22], vec![333]])
        );
        assert_eq!(
            vec![3, 1, 2],
            find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]])
        );
    }
}
