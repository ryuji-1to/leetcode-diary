pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .flat_map(|x| {
            x.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            vec![1, 3, 2, 5, 8, 3, 7, 7],
            separate_digits(vec![13, 25, 83, 77])
        );
        assert_eq!(vec![7, 1, 3, 9], separate_digits(vec![7, 1, 3, 9]));
    }
}
