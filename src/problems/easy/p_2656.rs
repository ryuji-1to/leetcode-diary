pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().max().unwrap() * k + (k * (k - 1)) / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(18, maximize_sum(vec![1, 2, 3, 4, 5], 3));
        assert_eq!(11, maximize_sum(vec![5, 5, 5], 2));
    }
}
