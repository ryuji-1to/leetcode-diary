pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    for i in 0..(nums.len() / 2 + 1) {
        if (&nums[i + 1..]).contains(&nums[i]) {
            return nums[i];
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, repeated_n_times(vec![1, 2, 3, 3]));
        assert_eq!(2, repeated_n_times(vec![2, 1, 2, 5, 3, 2]));
        assert_eq!(5, repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]));
    }
}
