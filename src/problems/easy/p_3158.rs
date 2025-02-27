pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
    let mut appears = Vec::<i32>::new();
    nums.iter().fold(0, |acc, &x| {
        if !appears.contains(&x) {
            appears.push(x);
            acc
        } else {
            acc ^ x
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, duplicate_numbers_xor(vec![1, 2, 1, 3]));
        assert_eq!(0, duplicate_numbers_xor(vec![1, 2, 3]));
        assert_eq!(3, duplicate_numbers_xor(vec![1, 2, 2, 1]));
    }
}
