pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut idx = 1;
    while idx <= arr.len() {
        result += arr
            .windows(idx)
            .fold(0, |acc, x| acc + x.iter().sum::<i32>());
        idx += 2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(58, sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]));
        assert_eq!(3, sum_odd_length_subarrays(vec![1, 2]));
        assert_eq!(66, sum_odd_length_subarrays(vec![10, 11, 12]));
    }
}
