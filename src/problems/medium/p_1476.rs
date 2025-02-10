struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle }
    }

    fn update_subrectanble(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..=row2 {
            for j in col1..=col2 {
                self.rectangle[i as usize][j as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works1() {
        let mut subrectangle_queries = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);
        let result = subrectangle_queries.get_value(0, 2);
        assert_eq!(1, result);

        subrectangle_queries.update_subrectanble(0, 0, 3, 2, 5);

        let result = subrectangle_queries.get_value(0, 2);
        assert_eq!(5, result);

        let result = subrectangle_queries.get_value(3, 1);
        assert_eq!(5, result);

        subrectangle_queries.update_subrectanble(3, 0, 3, 2, 10);

        let result = subrectangle_queries.get_value(3, 1);
        assert_eq!(10, result);

        let result = subrectangle_queries.get_value(0, 2);
        assert_eq!(5, result);
    }

    #[test]
    fn it_works2() {
        let mut subrectangle_queries =
            SubrectangleQueries::new(vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3]]);
        let result = subrectangle_queries.get_value(0, 0);
        assert_eq!(1, result);

        subrectangle_queries.update_subrectanble(0, 0, 2, 2, 100);

        let result = subrectangle_queries.get_value(0, 0);
        assert_eq!(100, result);

        let result = subrectangle_queries.get_value(2, 2);
        assert_eq!(100, result);

        subrectangle_queries.update_subrectanble(1, 1, 2, 2, 20);

        let result = subrectangle_queries.get_value(2, 2);
        assert_eq!(20, result);
    }
}
