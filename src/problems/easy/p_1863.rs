pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    //  fn dfs(nums: &Vec<i32>, index: usize, current_xor: i32) -> i32 {
    //     if index == nums.len() {
    //         return current_xor; // サブセットのXOR合計を返す
    //     }
    //     // 現在の要素を含めない場合と含める場合の合計を計算
    //     dfs(nums, index + 1, current_xor) + dfs(nums, index + 1, current_xor ^ nums[index])
    // }
    let n = (nums.len() - 1) as i32;
    nums.into_iter()
        .reduce(|total, num| total | num)
        .map(|total| total << n)
        .unwrap_or(0)
}
// 01 | 11 -> 11
// 01 -> 1
// 11 -> 3
// 01 ^ 11 -> 10 -> 2

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, subset_xor_sum(vec![1, 3]));
        assert_eq!(28, subset_xor_sum(vec![5, 1, 6]));
        assert_eq!(480, subset_xor_sum(vec![3, 4, 5, 6, 7, 8]));
    }
}
