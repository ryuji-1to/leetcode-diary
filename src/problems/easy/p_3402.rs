pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut tmp = vec![-1; grid[0].len()];
    for g in grid {
        for i in 0..g.len() {
            if tmp[i] >= g[i] {
                let diff = tmp[i] - g[i];
                result += diff + 1;
                tmp[i] = g[i] + diff + 1;
            } else {
                tmp[i] = g[i];
            }
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
            15,
            minimum_operations(vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]])
        );
        assert_eq!(
            12,
            minimum_operations(vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]])
        );
    }
}
