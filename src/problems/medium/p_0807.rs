pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    // let max_rows: Vec<i32> = grid.iter().map(|x| *x.iter().max().unwrap()).collect();
    // let max_cols: Vec<i32> = (0..n)
    //     .map(|j| (0..n).map(|i| grid[i][j]).max().unwrap())
    //     .collect();
    //
    // grid.iter()
    //     .enumerate()
    //     .map(|(i, row)| {
    //         row.iter()
    //             .enumerate()
    //             .map(|(j, &height)| max_rows[i].min(max_cols[j]) - height)
    //             .sum::<i32>()
    //     })
    //     .sum()
    let mut max_rows = vec![0; n];
    let mut max_cols = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            max_rows[i] = max_rows[i].max(grid[i][j]);
            max_cols[j] = max_cols[j].max(grid[i][j])
        }
    }
    let mut result = 0;
    for i in 0..n {
        for j in 0..n {
            result += max_rows[i].min(max_cols[j]) - grid[i][j];
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
            35,
            max_increase_keeping_skyline(vec![
                vec![3, 0, 8, 4],
                vec![2, 4, 5, 7],
                vec![9, 2, 6, 3],
                vec![0, 3, 1, 0]
            ])
        );
        assert_eq!(
            0,
            max_increase_keeping_skyline(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]])
        );
    }
}
