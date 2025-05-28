pub fn find_final_value(mut nums: Vec<i32>, mut original: i32) -> i32 {
    nums.sort_unstable();
    for v in nums {
        if v == original {
            original *= 2;
        }
    }
    original
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(24, find_final_value(vec![5, 3, 6, 1, 12], 3));
        assert_eq!(4, find_final_value(vec![2, 7, 9], 4));
    }
}
