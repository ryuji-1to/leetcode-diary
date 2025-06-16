pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    for i in 0..nums.len() {
        let left_sum: i32 = nums[..i].iter().sum();
        let right_sum = sum - left_sum - nums[i];
        if left_sum == right_sum {
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
        assert_eq!(3, find_middle_index(vec![2, 3, -1, 8, 4]));
        assert_eq!(2, find_middle_index(vec![1, -1, 4]));
        assert_eq!(-1, find_middle_index(vec![2, 5]));
    }
}
