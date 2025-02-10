pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort();
    students.sort();
    let mut result = 0;
    for i in 0..students.len() {
        result += students[i].abs_diff(seats[i]);
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, min_moves_to_seat(vec![3, 1, 5], vec![2, 7, 4]));
        assert_eq!(7, min_moves_to_seat(vec![4, 1, 5, 9], vec![1, 3, 2, 6]));
        assert_eq!(4, min_moves_to_seat(vec![2, 2, 6, 6], vec![1, 3, 2, 6]));
        assert_eq!(
            19,
            min_moves_to_seat(vec![12, 14, 19, 19, 12], vec![19, 2, 17, 20, 7])
        );
    }
}
