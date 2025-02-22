pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let current_xor = nums.iter().fold(0, |acc, x| acc ^ x);
    (current_xor ^ k).count_ones() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, min_operations(vec![2, 1, 3, 4], 1));
        assert_eq!(0, min_operations(vec![2, 0, 2, 0], 0))
    }
}
