pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        result += nums[i + 1..nums.len()]
            .iter()
            .filter(|&&x| x < target - nums[i])
            .count();
    }
    result as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, count_pairs(vec![-1, 1, 2, 3, 1], 2));
        assert_eq!(10, count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2));
    }
}
