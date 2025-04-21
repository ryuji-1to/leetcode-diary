pub fn sum_of_encrypted_int(nums: Vec<i32>) -> i32 {
    nums.iter()
        .map(|&x| {
            let x = x
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>();
            let max = *x.iter().max().unwrap() as i32;
            (0..x.len())
                .map(|i| 10_i32.pow(i as u32) * max)
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(6, sum_of_encrypted_int(vec![1, 2, 3]));
        assert_eq!(66, sum_of_encrypted_int(vec![10, 21, 31]));
        assert_eq!(999, sum_of_encrypted_int(vec![109]));
    }
}
