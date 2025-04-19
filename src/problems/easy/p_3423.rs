pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
    let mut result = nums[0].abs_diff(nums[nums.len() - 1]);
    for i in 0..nums.len() - 1 {
        let diff = nums[i].abs_diff(nums[i + 1]);
        if diff > result {
            result = diff;
        }
    }
    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, max_adjacent_distance(vec![1, 2, 4]));
        assert_eq!(5, max_adjacent_distance(vec![-5, -10, -5]));
    }
}
