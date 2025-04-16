pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut from_x = 0;
    let mut from_y = vec![0; grid[0].len()];
    let mut from_z = 0;
    for i in 0..grid.len() {
        let x = *grid[i].iter().max().unwrap();
        from_x += x;
        from_z += grid[i].iter().filter(|&&x| x > 0).count() as i32;
        for j in 0..grid[0].len() {
            let y = grid[i][j];
            if y > from_y[j] {
                from_y[j] = y;
            }
        }
    }
    from_z + from_x + from_y.iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(17, projection_area(vec![vec![1, 2], vec![3, 4]]));
        assert_eq!(5, projection_area(vec![vec![2]]));
        assert_eq!(8, projection_area(vec![vec![1, 0], vec![0, 2]]));
    }
}
