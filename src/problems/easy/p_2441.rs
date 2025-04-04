pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    for i in 0..nums.len() {
        if nums.contains(&(nums[i] * -1)) {
            return nums[i] * -1;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, find_max_k(vec![-1, 2, -3, 3]));
        assert_eq!(7, find_max_k(vec![-1, 10, 6, 7, -7, 1]));
        assert_eq!(-1, find_max_k(vec![-10, 8, 6, 7, -2, -3]));
    }
}
