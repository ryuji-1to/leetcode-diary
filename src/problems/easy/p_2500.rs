pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    let mut tmp = 0;
    for i in 0..grid[0].len() {
        for j in 0..grid.len() {
            if let Some(maximum) = grid[j].iter().cloned().max() {
                let position = grid[j].iter().position(|&x| x == maximum).unwrap();
                grid[j].remove(position);
                tmp = tmp.max(maximum)
            }
        }
        result += tmp;
        tmp = 0;
    }
    result
}
/*
pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
    for row in &mut grid {
        row.sort_unstable();
    }

    std::iter::from_fn(|| {
        grid.iter_mut().filter_map(|x| x.pop()).max()
    }).sum::<i32>()
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(8, delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]]));
        assert_eq!(10, delete_greatest_value(vec![vec![10]]));
    }
}
