pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let min_product = nums[0] * nums[1];
    let max_product = nums[nums.len() - 1] * nums[nums.len() - 2];
    max_product - min_product
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(34, max_product_difference(vec![5, 6, 2, 7, 4]));
        assert_eq!(64, max_product_difference(vec![4, 2, 5, 9, 7, 4, 8]));
    }
}
