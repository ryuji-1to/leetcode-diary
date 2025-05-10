pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    // let mut result = 0;
    //
    // for i in 0..nums.len() - 2 {
    //     for j in i + 1..nums.len() - 1 {
    //         for k in j + 1..nums.len() {
    //             if nums[i] != nums[j] && nums[i] != nums[k] && nums[j] != nums[k] {
    //                 result += 1;
    //             }
    //         }
    //     }
    // }
    //
    // result

    let mut freq = std::collections::HashMap::new();
    for &num in &nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut result = 0;
    let mut left = 0;
    let total = nums.len();

    for &count in freq.values() {
        let right = total - left - count;
        result += left * count * right;
        left += count;
    }

    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, unequal_triplets(vec![4, 4, 2, 4, 3]));
        assert_eq!(0, unequal_triplets(vec![1, 1, 1, 1, 1]));
    }
}
