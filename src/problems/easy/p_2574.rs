pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    // let mut left_sum = vec![0];
    // let mut right_sum = vec![0];
    // for i in 0..nums.len() - 1 {
    //     left_sum.push(nums[i] + left_sum[left_sum.len() - 1]);
    //     right_sum.insert(0, nums[nums.len() - 1 - i] + right_sum[0]);
    // }
    // left_sum
    //     .iter()
    //     .zip(right_sum)
    //     .map(|(&l, r)| l.abs_diff(r) as i32)
    //     .collect::<Vec<_>>()
    let total_sum: i32 = nums.iter().sum();
    let mut left_sum = 0;
    let mut result = Vec::new();

    for &num in &nums {
        let right_sum = total_sum - left_sum - num;
        result.push((left_sum - right_sum).abs());
        left_sum += num;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![15, 1, 11, 22],
            left_right_difference(vec![10, 4, 8, 3])
        );
        assert_eq!(vec![0], left_right_difference(vec![1]));
    }
}
