pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    points.windows(2).fold(0, |acc, x| {
        let x0 = x[0][0];
        let y0 = x[0][1];
        let x1 = x[1][0];
        let y1 = x[1][1];
        acc + ((x0 - x1).abs()).max((y0 - y1).abs())
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            7,
            min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]])
        );
        assert_eq!(
            5,
            min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]])
        );
    }
}
