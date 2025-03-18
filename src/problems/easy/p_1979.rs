pub fn find_gcd(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let min = nums[0];
    let max = nums[nums.len() - 1];
    for i in 0..min {
        if min % (min - i) == 0 && max % (min - i) == 0 {
            return min - i;
        }
    }
    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, find_gcd(vec![2, 5, 6, 9, 10]));
        assert_eq!(1, find_gcd(vec![7, 5, 6, 8, 3]));
        assert_eq!(3, find_gcd(vec![3, 3]));
    }
}
