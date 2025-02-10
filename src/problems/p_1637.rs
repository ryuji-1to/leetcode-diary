pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
    // let mut x = points.iter().map(|v| v[0]).collect::<Vec<i32>>();
    // x.sort();
    // let mut result = 0;
    // for i in 0..(x.len() - 1) {
    //     if x[i + 1] - x[i] > result {
    //         result = x[i + 1] - x[i];
    //     }
    // }
    // result
    points.sort_unstable_by_key(|p| p[0]);
    points
        .windows(2)
        .map(|pair| pair[1][0] - pair[0][0])
        .max()
        .unwrap_or(0)
}
