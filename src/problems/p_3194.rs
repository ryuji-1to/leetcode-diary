pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
    let mut result = f64::MAX;
    nums.sort_unstable();
    for i in 0..nums.len() / 2 {
        let minimum = f64::from(nums[i]);
        let maximum = f64::from(nums[nums.len() - 1 - i]);
        result = result.min((minimum + maximum) / 2.0);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5.5, minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]));
        assert_eq!(5.5, minimum_average(vec![1, 9, 8, 3, 10, 5]));
        assert_eq!(5.0, minimum_average(vec![1, 2, 3, 7, 8, 9]));
    }
}
