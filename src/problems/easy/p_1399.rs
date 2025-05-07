pub fn count_largest_group(n: i32) -> i32 {
    // use std::collections::HashMap;
    let mut tmp = vec![0; 35];
    for mut x in 1..=n {
        let mut digit_sum = 0;
        while x > 0 {
            digit_sum += x % 10;
            x /= 10;
        }
        tmp[(digit_sum - 1) as usize] += 1;
    }
    let max = *tmp.iter().max().unwrap();
    tmp.iter().filter(|&&x| x == max).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, count_largest_group(13));
        assert_eq!(2, count_largest_group(2));
    }
}
