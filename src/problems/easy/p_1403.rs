pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
    nums.sort_unstable_by(|a, b| b.cmp(a));
    let total = nums.iter().sum::<i32>();
    let mut sum = 0;
    let mut result = vec![];
    for v in nums {
        result.push(v);
        sum += v;
        if (sum > total / 2) {
            break;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![10, 9], min_subsequence(vec![4, 3, 10, 9, 8]));
        assert_eq!(vec![7, 7, 6], min_subsequence(vec![4, 4, 7, 6, 7]));
        assert_eq!(
            vec![9, 8, 8],
            min_subsequence(vec![1, 7, 4, 7, 1, 9, 4, 8, 8])
        )
    }
}
