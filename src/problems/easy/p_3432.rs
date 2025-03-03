pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() - 1 {
        let left: i32 = nums[..=i].iter().sum();
        let right: i32 = nums[i + 1..].iter().sum();
        if (left - right) % 2 == 0 {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_partitions(vec![10, 10, 3, 7, 6]));
        assert_eq!(0, count_partitions(vec![1, 2, 2]));
        assert_eq!(3, count_partitions(vec![2, 4, 6, 8]));
    }
}
