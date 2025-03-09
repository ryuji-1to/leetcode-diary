// 等差数列のサブ配列チェック問題
//
// 問題要約:
// - 数列が「等差数列」であるとは、連続する要素間の差が一定であること
// - 与えられた配列 nums と複数のクエリ [l[i], r[i]] に対して、各クエリが示すサブ配列が
//   「並べ替えた後に」等差数列となるかどうかを判定する
pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    (0..l.len())
        .map(|i| {
            let start = l[i] as usize;
            let end = r[i] as usize;
            if end - start + 1 <= 2 {
                true
            } else {
                let mut target = nums[start..=end].to_vec();
                target.sort_unstable();
                let diff = target[1] - target[0];
                target.windows(2).all(|x| x[1] - x[0] == diff)
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![true, false, true],
            check_arithmetic_subarrays(vec![4, 6, 5, 9, 3, 7], vec![0, 0, 2], vec![2, 3, 5])
        );
        assert_eq!(
            vec![false, true, false, false, true, true],
            check_arithmetic_subarrays(
                vec![-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                vec![0, 1, 6, 4, 8, 7],
                vec![4, 4, 9, 7, 9, 10]
            )
        );
    }
}
