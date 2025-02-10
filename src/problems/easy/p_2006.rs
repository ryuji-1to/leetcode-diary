pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if (nums[i] - nums[j]).abs() == k {
                result += 1;
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
        assert_eq!(4, count_k_difference(vec![1, 2, 2, 1], 1));
        assert_eq!(0, count_k_difference(vec![1, 3], 3));
        assert_eq!(3, count_k_difference(vec![3, 2, 1, 5, 4], 2));
    }
}
