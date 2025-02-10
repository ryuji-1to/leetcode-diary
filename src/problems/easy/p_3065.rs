pub fn min_operation(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter()
        .fold(0, |acc, &x| if x < k { acc + 1 } else { acc })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, min_operation(vec![2, 11, 10, 1, 3], 10));
        assert_eq!(0, min_operation(vec![1, 1, 2, 4, 9], 1));
        assert_eq!(4, min_operation(vec![1, 1, 2, 4, 9], 9));
    }
}
