pub fn is_array_special(nums: Vec<i32>) -> bool {
    nums.windows(2)
        .all(|c| (c[0] % 2 == 1 && c[1] % 2 == 0) || (c[0] % 2 == 0 && c[1] % 2 == 1))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, is_array_special(vec![1]));
        assert_eq!(true, is_array_special(vec![2, 1, 4]));
        assert_eq!(false, is_array_special(vec![4, 3, 1, 6]));
    }
}
