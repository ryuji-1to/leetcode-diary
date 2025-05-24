pub fn smallest_index(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .enumerate()
        .find(|(i, mut n)| {
            let mut sum = 0;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            *i == sum as usize
        })
        .map(|(i, _)| i as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, smallest_index(vec![1, 3, 2]));
        assert_eq!(1, smallest_index(vec![1, 10, 11]));
    }
}
