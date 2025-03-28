pub fn find_non_min_or_max(mut nums: Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return -1;
    }
    nums.sort_unstable();
    nums[1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, find_non_min_or_max(vec![3, 2, 1, 4]));
        assert_eq!(-1, find_non_min_or_max(vec![1, 2]));
        assert_eq!(2, find_non_min_or_max(vec![2, 1, 3]));
    }
}
