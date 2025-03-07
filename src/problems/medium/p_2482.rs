pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let rows_count = grid.len();
    let cols_count = grid[0].len();
    let mut result = Vec::with_capacity(rows_count);
    let mut cols_cache = Vec::<(i32, i32)>::with_capacity(cols_count);

    for i in 0..rows_count {
        let mut tmp = Vec::new();
        let (ones_row, zeros_row) = grid[i].iter().fold((0, 0), |acc, &x| {
            if x == 0 {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + 1, acc.1)
            }
        });
        for j in 0..cols_count {
            if cols_cache.get(j).is_none() {
                let cache = (0..rows_count).fold((0, 0), |acc, i| {
                    if grid[i][j] == 0 {
                        (acc.0, acc.1 + 1)
                    } else {
                        (acc.0 + 1, acc.1)
                    }
                });
                cols_cache.push(cache);
            }
            let (ones_col, zeros_col) = cols_cache[j];
            tmp.push(ones_row + ones_col - zeros_row - zeros_col);
        }
        result.push(tmp);
    }

    result
}

/*
pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rows = vec![0;grid.len()];
    let mut cols = vec![0; grid[0].len()];
    // ones_rowとones_colを求める
    for row in 0..grid.len(){
        for col in 0..grid[0].len(){
            rows[row]+=grid[row][col];
            cols[col]+= grid[row][col];
                }
            }

    for r in 0..rows.len(){
        for c in 0..cols.len(){
            // 求めたones_rowとcols_rowを使用して0の数を求めてgridを作成
            grid[r][c] = rows[r] - (rows.len() as i32-rows[r]) + cols[c] - (cols.len() as i32-cols[c]);
        }
    }
    grid
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![0, 0, 4], vec![0, 0, 4], vec![-2, -2, 2]],
            ones_minus_zeros(vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]])
        );
        assert_eq!(
            vec![vec![5, 5, 5], vec![5, 5, 5]],
            ones_minus_zeros(vec![vec![1, 1, 1], vec![1, 1, 1]])
        );
    }
}
