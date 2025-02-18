pub fn count_points(points: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result = Vec::new();
    // (x - a)^2 + (y - a)^2 <= r^2 かどうか
    for v in queries {
        let mut count = 0;
        for p in &points {
            if (v[0] - p[0]).pow(2) + (v[1] - p[1]).pow(2) <= v[2].pow(2) {
                count += 1
            }
        }
        result.push(count);
        count = 0;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![3, 2, 2],
            count_points(
                vec![vec![1, 3], vec![3, 3], vec![5, 3], vec![2, 2]],
                vec![vec![2, 3, 1], vec![4, 3, 1], vec![1, 1, 2]]
            )
        );
        assert_eq!(
            vec![2, 3, 2, 4],
            count_points(
                vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]],
                vec![vec![1, 2, 2], vec![2, 2, 2], vec![4, 3, 2], vec![4, 3, 3]]
            )
        );
    }
}
