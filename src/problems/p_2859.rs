pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    // let mut result = 0;
    // for i in 0..nums.len() {
    //     if i.count_ones() == k as u32 {
    //         result += nums[i];
    //     }
    // }
    // result
    nums.iter().into_iter().enumerate().fold(0, |acc, (i, x)| {
        if i.count_ones() == k as u32 {
            acc + x
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(13, sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1));
        assert_eq!(1, sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2));
    }
}
