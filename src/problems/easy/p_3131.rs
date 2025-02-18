pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    nums2.iter().min().unwrap() - nums1.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, added_integer(vec![2, 6, 4], vec![9, 7, 5]));
        assert_eq!(-5, added_integer(vec![10], vec![5]));
        assert_eq!(0, added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]));
    }
}
