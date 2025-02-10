pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = grid.len();
    let mut max_local = vec![vec![0; n - 2]; n - 2];

    // i,jでそれぞれ４隅を求める
    for i in 0..n - 2 {
        for j in 0..n - 2 {
            // それぞれのmax_valueを↓のループで求める
            let mut max_value = i32::MIN;
            for x in i..i + 3 {
                for y in j..j + 3 {
                    max_value = max_value.max(grid[x][y]);
                }
            }
            max_local[i][j] = max_value;
        }
    }

    max_local
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![vec![9, 9], vec![8, 6]],
            largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ])
        );
    }
}
