pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut nums = nums
        .iter()
        .filter(|&&x| x > 0)
        .collect::<std::collections::HashSet<_>>();
    nums.iter().count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, minimum_operations(vec![1, 5, 0, 3, 5]));
        assert_eq!(0, minimum_operations(vec![0]));
    }
}
