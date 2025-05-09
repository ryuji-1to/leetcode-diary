pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
    // let grid = grid
    //     .iter()
    //     .map(|g| g.iter().filter(|&&x| x == 1).count())
    //     .collect::<Vec<_>>();
    // grid.iter()
    //     .position(|g| g == grid.iter().max().unwrap())
    //     .unwrap() as i32
    let mut team = 0;
    let mut score = 0;
    for i in 0..grid.len() {
        let ones = grid[i].iter().filter(|&&x| x == 1).count();
        if ones > score {
            team = i;
            score = ones;
        }
    }
    team as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, find_champion(vec![vec![0, 1], vec![0, 0]]));
        assert_eq!(
            1,
            find_champion(vec![vec![0, 0, 1], vec![1, 0, 1], vec![0, 0, 0]])
        );
    }
}
