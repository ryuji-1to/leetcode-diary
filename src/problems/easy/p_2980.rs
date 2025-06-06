pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
    nums.iter().filter(|&&x| x % 2 == 0).count() >= 2
    // let mut even = 0;
    // for n in nums {
    //     if n % 2 == 0 {
    //         even += 1;
    //     }
    //     if even >= 2 {
    //         return true;
    //     }
    // }
    // false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, has_trailing_zeros(vec![1, 2, 3, 4, 5]));
        assert_eq!(true, has_trailing_zeros(vec![2, 4, 8, 16]));
        assert_eq!(false, has_trailing_zeros(vec![1, 3, 5, 7, 9]));
    }
}
