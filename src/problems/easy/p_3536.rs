pub fn max_product(n: i32) -> i32 {
    // let mut digits = n
    //     .to_string()
    //     .chars()
    //     .map(|x| x.to_digit(10).unwrap() as i32)
    //     .collect::<Vec<_>>();
    // digits.sort_unstable_by(|a, b| b.cmp(a));
    // digits[0] * digits[1]
    let mut digits: Vec<_> = std::iter::successors(Some(n), |&x| (x >= 10).then(|| x / 10))
        .map(|x| x % 10)
        .collect();
    digits.sort_unstable_by(|a, b| b.cmp(a));
    digits[0] * digits[1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, max_product(31));
        assert_eq!(4, max_product(22));
        assert_eq!(8, max_product(124));
    }
}
