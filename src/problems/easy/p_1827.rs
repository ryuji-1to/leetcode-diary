pub fn min_operations(mut nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() - 1 {
        if nums[i] >= nums[i + 1] {
            let t = (nums[i] - nums[i + 1]) + 1;
            nums[i + 1] += t;
            result += t;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, min_operations(vec![1, 1, 1]));
        assert_eq!(14, min_operations(vec![1, 5, 2, 4, 1]));
        assert_eq!(0, min_operations(vec![8]));
    }
}
