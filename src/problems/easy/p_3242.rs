struct NeighborSum {
    grid: Vec<Vec<i32>>,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { grid }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let (raw, col) = self.get_target_index(value);
        let left = self.get_target_value((raw, col - 1));
        let right = self.get_target_value((raw, col + 1));
        let top = self.get_target_value((raw - 1, col));
        let bottom = self.get_target_value((raw + 1, col));
        left + right + top + bottom
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let (raw, col) = self.get_target_index(value);
        let left_top = self.get_target_value((raw - 1, col - 1));
        let right_top = self.get_target_value((raw - 1, col + 1));
        let left_bottom = self.get_target_value((raw + 1, col - 1));
        let right_bottom = self.get_target_value((raw + 1, col + 1));
        left_top + right_top + left_bottom + right_bottom
    }

    fn get_target_index(&self, value: i32) -> (i32, i32) {
        let mut result = (0, 0);
        for i in 0..self.grid.len() {
            if self.grid[i].contains(&value) {
                let col = self.grid[i].iter().position(|&x| x == value).unwrap();
                result = (i as i32, col as i32);
                break;
            }
        }
        result
    }

    fn get_target_value(&self, place: (i32, i32)) -> i32 {
        let (raw, col) = place;
        if raw < 0
            || col < 0
            || raw as usize > self.grid.len() - 1
            || col as usize > self.grid.len() - 1
        {
            return 0;
        }
        self.grid[raw as usize][col as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works1() {
        let grid = NeighborSum::new(vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]]);
        assert_eq!(6, grid.adjacent_sum(1));
        assert_eq!(16, grid.adjacent_sum(4));
        assert_eq!(16, grid.diagonal_sum(4));
        assert_eq!(4, grid.diagonal_sum(8));
    }

    #[test]
    fn it_works2() {
        let grid = NeighborSum::new(vec![
            vec![1, 2, 0, 3],
            vec![4, 7, 15, 6],
            vec![8, 9, 10, 11],
            vec![12, 13, 14, 5],
        ]);
        assert_eq!(23, grid.adjacent_sum(15));
        assert_eq!(45, grid.diagonal_sum(9));
    }
}
