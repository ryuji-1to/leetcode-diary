pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .map(|&x| {
            if x % 2 == 0 {
                return -1;
            }
            for i in 1..x {
                if (i | (i + 1)) == x {
                    return i;
                }
            }
            -1
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![-1, 1, 4, 3], min_bitwise_array(vec![2, 3, 5, 7]));
        assert_eq!(vec![9, 12, 15], min_bitwise_array(vec![11, 13, 31]));
    }
}
