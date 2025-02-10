pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut result = Vec::new();
    for mut v in nums {
        let mut sum_of_digits = 0;
        while v > 0 {
            sum_of_digits += v % 10;
            v /= 10;
        }
        result.push(sum_of_digits);
    }
    *result.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, min_element(vec![10, 12, 13, 14]));
        assert_eq!(1, min_element(vec![1, 2, 3, 4]));
        assert_eq!(10, min_element(vec![999, 19, 199]));
    }
}
