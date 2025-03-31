pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    let mut tmp = vec![0; 101];
    for &n in &nums {
        tmp[n as usize] += 1;
    }
    let mut result = vec![0, 0];
    for n in tmp {
        result[0] += n / 2;
        result[1] += n % 2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(vec![3, 1], number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]));
        assert_eq!(vec![1, 0], number_of_pairs(vec![1, 1]));
        assert_eq!(vec![0, 1], number_of_pairs(vec![0]));
    }
}
