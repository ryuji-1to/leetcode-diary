pub fn count_digits(num: i32) -> i32 {
    let mut target = num;
    let mut result = 0;
    while target != 0 {
        let digit = target % 10;
        if num % digit == 0 {
            result += 1;
        }
        target /= 10;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, count_digits(7));
        assert_eq!(2, count_digits(121));
        assert_eq!(4, count_digits(1248));
        assert_eq!(0, count_digits(54));
    }
}
