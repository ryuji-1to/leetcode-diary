pub fn subarray_sum(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        let start = (i as i32 - nums[i]).max(0);
        result += &nums[(start as usize)..i + 1].iter().sum();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(11, subarray_sum(vec![2, 3, 1]));
        assert_eq!(13, subarray_sum(vec![3, 1, 1, 2]));
    }
}
