pub fn alternate_digit_sum(n: i32) -> i32 {
    n.to_string().chars().enumerate().fold(0, |acc, (i, x)| {
        let x = x.to_digit(10).unwrap() as i32;
        if i % 2 == 0 {
            acc + x
        } else {
            acc - x
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, alternate_digit_sum(521));
        assert_eq!(1, alternate_digit_sum(111));
        assert_eq!(0, alternate_digit_sum(886996));
    }
}
