pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
    let mut sum_of_digits = 0;
    let mut _x = x;
    while _x > 0 {
        sum_of_digits += _x % 10;
        _x /= 10;
    }
    if x % sum_of_digits == 0 {
        sum_of_digits
    } else {
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(9, sum_of_the_digits_of_harshad_number(18));
        assert_eq!(-1, sum_of_the_digits_of_harshad_number(23));
    }
}
