pub fn smallest_equal(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if (i as i32 % 10 == nums[i]) {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(0, smallest_equal(vec![0, 1, 2]));
        assert_eq!(2, smallest_equal(vec![4, 3, 2, 1]));
        assert_eq!(-1, smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    }
}
