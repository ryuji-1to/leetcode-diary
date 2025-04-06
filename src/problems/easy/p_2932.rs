pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i].abs_diff(nums[j]) as i32 > nums[i].min(nums[j]) {
                continue;
            }
            if nums[i] ^ nums[j] > result {
                result = nums[i] ^ nums[j];
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(7, maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, maximum_strong_pair_xor(vec![10, 100]));
        assert_eq!(7, maximum_strong_pair_xor(vec![5, 6, 25, 30]));
    }
}
