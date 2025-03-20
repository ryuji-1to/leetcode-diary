pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut tmp = vec![0; 100];
    for v in nums {
        tmp[(v - 1) as usize] += 1;
    }
    let maximum = *tmp.iter().max().unwrap();
    tmp.iter().filter(|&&x| x == maximum).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, max_frequency_elements(vec![1, 2, 2, 3, 1, 4]));
        assert_eq!(5, max_frequency_elements(vec![1, 2, 3, 4, 5]));
    }
}
