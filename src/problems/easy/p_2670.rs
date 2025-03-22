use std::collections::HashSet;

pub fn distinct_difference_array(nums: Vec<i32>) -> Vec<i32> {
    /*
    let mut result = Vec::new();
    for i in 0..nums.len() {
        let prefix = &nums[..=i];
        let suffix = &nums[i + 1..nums.len()];
        let prefix_set = HashSet::<&i32>::from_iter(prefix.iter());
        let suffix_set = HashSet::<&i32>::from_iter(suffix.iter());
        result.push(prefix_set.len() as i32 - suffix_set.len() as i32);
    }
    result
    */
    let n = nums.len();
    let mut result = vec![0; n];

    let mut prefix_set = HashSet::new();
    let mut suffix_set = HashSet::new();
    let mut suffix_count = vec![0; n + 1]; // 後半の distinct 数を事前に計算

    // 後ろから suffix のユニーク数を計算
    for i in (0..n).rev() {
        suffix_set.insert(nums[i]);
        suffix_count[i] = suffix_set.len();
    }

    // 前から prefix のユニーク数を計算しつつ、結果を埋める
    for i in 0..n {
        prefix_set.insert(nums[i]);
        result[i] = prefix_set.len() as i32 - suffix_count[i + 1] as i32;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![-3, -1, 1, 3, 5],
            distinct_difference_array(vec![1, 2, 3, 4, 5])
        );
        assert_eq!(
            vec![-2, -1, 0, 2, 3],
            distinct_difference_array(vec![3, 2, 3, 4, 2])
        );
    }
}
