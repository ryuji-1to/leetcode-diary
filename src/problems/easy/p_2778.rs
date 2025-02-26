pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
    nums.iter().enumerate().fold(0, |acc, (i, x)| {
        if nums.len() % (i + 1) == 0 {
            acc + x * x
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(21, sum_of_squares(vec![1, 2, 3, 4]));
        assert_eq!(63, sum_of_squares(vec![2, 7, 1, 19, 18, 3]));
    }
}
