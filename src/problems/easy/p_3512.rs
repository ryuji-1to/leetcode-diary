pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().sum::<i32>() % k
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, min_operations(vec![3, 9, 7], 5));
        assert_eq!(0, min_operations(vec![4, 1, 3], 4));
    }
}
