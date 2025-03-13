pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut row_increment = vec![0; m as usize];
    let mut col_increment = vec![0; n as usize];

    for index in indices {
        row_increment[index[0] as usize] += 1;
        col_increment[index[1] as usize] += 1;
    }

    // let mut odd = 0;
    // for i in 0..m as usize {
    //     for j in 0..n as usize {
    //         let cell_value = row_increment[i] + col_increment[j];
    //         if cell_value % 2 == 1 {
    //             odd += 1;
    //         }
    //     }
    // }
    let odd_rows = row_increment
        .iter()
        .filter(|&&count| count % 2 == 1)
        .count() as i32;
    let odd_cols = col_increment
        .iter()
        .filter(|&&count| count % 2 == 1)
        .count() as i32;

    // 奇数値のセルの総数を計算
    // 奇数行 × 偶数列 + 偶数行 × 奇数列
    let even_rows = m - odd_rows;
    let even_cols = n - odd_cols;

    (odd_rows * even_cols) + (even_rows * odd_cols)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]));
        assert_eq!(0, odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]));
    }
}
