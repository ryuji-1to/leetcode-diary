pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut count = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            if nums[i] == nums[j] && (i * j) % k as usize == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_pairs(vec![3, 1, 2, 2, 2, 1, 3], 2));
        assert_eq!(0, count_pairs(vec![1, 2, 3, 4], 1));
    }
}
